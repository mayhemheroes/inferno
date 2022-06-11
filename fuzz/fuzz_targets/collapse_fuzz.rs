#![no_main]
use inferno::collapse::guess::Folder as Guess;
use inferno::collapse::sample::Folder as Sample;
use inferno::collapse::vsprof::Folder as Vsprof;
use inferno::collapse::vtune::Folder as Vtune;
use inferno::collapse::Collapse;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    fuzz_folder(data, Guess::default());
    fuzz_folder(data, Sample::default());
    fuzz_folder(data, Vsprof::default());
    fuzz_folder(data, Vtune::default());
});

fn fuzz_folder(data: &str, mut folder: impl Collapse) {
    let _ = folder.is_applicable(data);

    let sink = std::io::sink();
    let cursor = std::io::Cursor::new(data);
    let _ = folder.collapse(cursor, sink);
}
