use env_logger::Env;
use kiri::{
    evdev_keys::*, AddLayer, Key, KeyConfig, KeyConfigRun, KeyInput, PairRemapEntry, RemapLayer,
    SingleRemapEntry,
};
use std::iter;

fn config_simple_remap() -> RemapLayer<()> {
    let keys: &[(Key, Key)] = &[
        (KEY_CAPSLOCK, KEY_LEFTCTRL),
        (KEY_HENKAN, KEY_ENTER),
        (KEY_MUHENKAN, KEY_BACKSPACE),
    ];
    RemapLayer {
        pair_remap_entries: vec![PairRemapEntry {
            condition: (),
            input: [KeyInput::press(KEY_K), KeyInput::press(KEY_L)],
            output: vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_RO),
                KeyInput::release(KEY_RO),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            transition: (),
            threshold: 60,
        }],
        single_remap_entries: keys
            .iter()
            .map(|(i, o)| SingleRemapEntry {
                condition: (),
                input: KeyInput::press(*i),
                output: vec![KeyInput::press(*o)],
                transition: (),
            })
            .chain(keys.iter().map(|(i, o)| SingleRemapEntry {
                condition: (),
                input: KeyInput::release(*i),
                output: vec![KeyInput::release(*o)],
                transition: (),
            }))
            .collect(),
        layer_name: "simple remap",
        initial_state: (),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateSands {
    Normal,
    Space,
    Shift,
}

fn config_sands() -> RemapLayer<StateSands> {
    use StateSands::*;
    #[allow(clippy::type_complexity)]
    let config: &[(&[StateSands], KeyInput, &[KeyInput], Option<StateSands>)] = &[
        (
            &[Normal],
            KeyInput::press(KEY_SPACE),
            &[KeyInput::press(KEY_LEFTSHIFT)],
            Some(Space),
        ),
        (&[Space, Shift], KeyInput::press(KEY_SPACE), &[], None),
        (
            &[Space],
            KeyInput::release(KEY_SPACE),
            &[
                KeyInput::release(KEY_LEFTSHIFT),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
            ],
            Some(Normal),
        ),
        (
            &[Shift],
            KeyInput::release(KEY_SPACE),
            &[KeyInput::release(KEY_LEFTSHIFT)],
            Some(Normal),
        ),
    ];
    let config = config.iter().flat_map(|(cs, i, o, t)| {
        cs.iter().map(move |c| SingleRemapEntry {
            condition: *c,
            input: *i,
            output: o.to_vec(),
            transition: t.unwrap_or(*c),
        })
    });
    let config2 = all_keys()
        .filter(|k| *k != KEY_SPACE)
        .map(|k| SingleRemapEntry {
            condition: Space,
            input: KeyInput::press(k),
            output: vec![KeyInput::press(k)],
            transition: Shift,
        });
    RemapLayer {
        pair_remap_entries: Vec::new(),
        single_remap_entries: config.chain(config2).collect(),
        layer_name: "SandS",
        initial_state: Normal,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateShiftRealease {
    Normal,
    Shift,
    ContinuousShift,
}

fn config_wait_slow_shift_release() -> RemapLayer<StateShiftRealease> {
    use StateShiftRealease::*;
    let ps = all_keys()
        .filter(|k| *k != KEY_LEFTSHIFT && *k != KEY_SPACE)
        .map(|k| PairRemapEntry {
            condition: ContinuousShift,
            input: [KeyInput::release(KEY_LEFTSHIFT), KeyInput::press(k)],
            output: vec![KeyInput::release(KEY_LEFTSHIFT), KeyInput::press(k)],
            transition: Normal,
            threshold: 120,
        })
        .collect();
    let ss = all_keys()
        .filter(|k| *k != KEY_LEFTSHIFT)
        .map(|k| SingleRemapEntry {
            condition: Shift,
            input: KeyInput::press(k),
            output: vec![KeyInput::press(k)],
            transition: ContinuousShift,
        });
    RemapLayer {
        pair_remap_entries: ps,
        single_remap_entries: ss
            .chain(iter::once(SingleRemapEntry {
                condition: Normal,
                input: KeyInput::press(KEY_LEFTSHIFT),
                output: vec![KeyInput::press(KEY_LEFTSHIFT)],
                transition: Shift,
            }))
            .chain(iter::once(SingleRemapEntry {
                condition: Shift,
                input: KeyInput::release(KEY_LEFTSHIFT),
                output: vec![KeyInput::release(KEY_LEFTSHIFT)],
                transition: Normal,
            }))
            .chain(iter::once(SingleRemapEntry {
                condition: ContinuousShift,
                input: KeyInput::release(KEY_LEFTSHIFT),
                output: vec![KeyInput::release(KEY_LEFTSHIFT)],
                transition: Normal,
            }))
            .collect(),
        layer_name: "wait slow shift release",
        initial_state: Normal,
    }
}

fn config_swap_a_b() -> RemapLayer<()> {
    RemapLayer {
        pair_remap_entries: Vec::new(),
        single_remap_entries: vec![
            SingleRemapEntry {
                condition: (),
                input: KeyInput::press(KEY_A),
                output: vec![KeyInput::press(KEY_B)],
                transition: (),
            },
            SingleRemapEntry {
                condition: (),
                input: KeyInput::press(KEY_B),
                output: vec![KeyInput::press(KEY_A)],
                transition: (),
            },
        ],
        layer_name: "a to b",
        initial_state: (),
    }
}

fn main() {
    // Config for log messages
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    KeyConfig::default()
        // Remap:
        // - `capslock` to `ctrl`
        // - `henkan` to `enter` (only works with JIS keyboards)
        // - `muhenkan` to `backspace` (only works with JIS keyboards)
        .add_layer(config_simple_remap())
        // Shift while holding `space`
        .add_layer(config_sands())
        // Prevent things like THis
        .add_layer(config_wait_slow_shift_release())
        // Swap `a` and `b`
        .add_layer(config_swap_a_b())
        // Swap `a` and `b` again so nothing happened. (this illustrates how the layers work)
        .add_layer(config_swap_a_b())
        // Terminate this program immediately when KEY_CALC is pressed.
        .emergency_stop_key(KEY_CALC)
        // Run the remapper
        .run();
}
