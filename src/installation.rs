use crate::utils::{bonus_inst, inst, manual_install, print_res};
use crate::{Distro, Infos};
use question::{Answer, Question};
use std::process::Command;

pub fn default_installation(distro: Distro) {
    let info = Infos {
        distro: &distro,
        username: String::new(),
    };
    inst_git(&distro);
    inst(&distro, "nodejs");
    inst(&distro, "npm");
    inst_nvim(&info);
    inst_zsh(&distro);
    inst(&distro, "github-cli");
    if distro == Distro::Debian {
        inst(&distro, "clangd-11");
    } else {
        inst(&distro, "llvm");
    }
    inst(&distro, "curl");
    inst(&distro, "make");
    // Bonus
    bonus_inst(&distro, "godot");
    bonus_inst(&distro, "gimp");
    while manual_install(&distro) {}
    folders();
}

//pub fn inst_dashlane_cli() {}
pub fn folders() {
    if Question::new("Do you want the populate the classics' folders ??").confirm() == Answer::NO {
        return;
    }
    // TODO ca ne marche pas vraiment..
    let home: String = format!("/home/{}/", whoami::username());
    let _cmd = Command::new("rmdir")
        .arg(format!("{home}/*"))
        .output()
        .expect("Il ne veut pas supprimer les home/folders*");
    for dir in vec!["Download", "Projects", "Cursus42", "Games"] {
        let _cmd = Command::new("mkdir")
            .arg(format!("{home}/{dir}"))
            .output()
            .expect("Il ne veut pas supprimer les home/folders*");
    }
}

pub fn custom_installation(distro: Distro) {
    todo!("custom to finish !!! {:?}", distro);
}

// TODO un peu chaud de faire des pipes pour l'instant
// Ca va rester sur du shell
/*
fn inst_rust() {
    let cmd = Command::new("sudo")
        .arg("curl")
        .args(["--proto", "'=https'"])
        .args(["--tlsv1.2", "-sSf", "https://sh.rustup.rs", "|", "sh"])
        .output()
        .expect("Rust Install fail");
    print_res(&cmd, "Rust installed !");
}
*/

fn inst_nvim(infos: &Infos) {
    if infos.distro == &Distro::Debian {
        let cmd = Command::new("sudo")
            .arg("add-apt-repository")
            .arg("-y")
            .arg("ppa:neovim-ppa/stable")
            .output()
            .expect("get la last v de nvim");
        print_res(&cmd, "nvim getted from last version");
        let cmd = Command::new("sudo")
            .arg("apt-get")
            .arg("-y")
            .arg("update")
            .output()
            .expect("Il ne veut pas supprimer la config nvim");
        print_res(&cmd, "updated");
    }
    inst(&infos.distro, "neovim");
    let home: String = format!("/home/{}/", whoami::username());
    let nvim = format!("{home}/.config/nvim");
    let _cmd = Command::new("rm")
        .arg("-rf")
        .arg(&nvim)
        .output()
        .expect("Il ne veut pas supprimer la config nvim");
    let cmd = Command::new("git")
        .arg("clone")
        .arg("https://github.com/QuentinPoto/nvim_config.git")
        .arg(&nvim)
        .output()
        .expect("Il ne veut pas git clone ma config nvim");
    print_res(&cmd, "Git config cloned");
    let _cmd = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("https://github.com/wbthomason/packer.nvim")
        .arg(format!(
            "{home}/.local/share/nvim/site/pack/packer/start/packer.nvim"
        ))
        .output()
        .expect("Il ne veut pas git clone ma config nvim");
    let cmd = Command::new("git")
        .arg("clone")
        .arg("https://github.com/github/copilot.vim")
        .arg(format!("{home}/.config/nvim/pack/github/start/copilot.vim"))
        .output()
        .expect("Il ne veut pas git clone packer");
    print_res(&cmd, "Github copilot installed");
}

fn inst_zsh(distro: &Distro) {
    inst(distro, "zsh");
    // TODO : zsh en main
}
fn inst_git(distro: &Distro) {
    inst(distro, "git");
    let _cmd = Command::new("git")
        .args(["config", "--global", "user.name", "Quentin", "Jungo"])
        .output()
        .expect("Il ne veut pas git clone packer");
    let _cmd = Command::new("git")
        .args([
            "config",
            "--global",
            "user.email",
            "quentin.jungo@gmail.com",
        ])
        .output()
        .expect("Il ne veut pas git clone packer");
}
