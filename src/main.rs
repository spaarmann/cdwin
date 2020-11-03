use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wincd", about = "Navigate to a Windows path in WSL.")]
struct Opt {
    #[structopt(name = "PATH")]
    path: String,
}

fn main() {
    let opt = Opt::from_args();

    // This is kind of horrible. It should use proper path manipulation
    // APIs and do validation, and error handling, etc.
    // But the std path handling seems to be platform specific in a way
    // that means I can't easily handle Windows paths when running on Linux,
    // and I can't be bothered to figure that out right now, so here we are.

    // Assume a path is roughly of the form C:\some_amount_of_dirs\ and we can
    // iterate over it by codepoint.
    let mut chars = opt.path.chars();

    // drive letter
    let drive = chars.next().expect("Path is empty.");
    // the ':', just skip it
    chars.next().expect("Not a valid path");
    // the remainder is essentially the path we want, except for changing \ to /

    let mut new_path = "/mnt/".to_string();
    // drive letter is usually uppercase in a Windows path, but the /mnt/ dir is
    // lowercase
    new_path.push(drive.to_lowercase().next().expect("What the hell is this drive letter"));

    // prefix of result path constructed, now just push all the rest with the slash
    // substitution.

    for c in chars {
        if c == '\\' {                
            new_path.push('/');
        }
        else {
            new_path.push(c);
        }
    }

    println!("{}", new_path);
}
