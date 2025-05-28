// 📂 src/utils/format.rs

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// 🎨 Imprime o banner inicial da FireChain CLI
pub fn print_banner() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // 🔥 Título estilizado
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
    writeln!(&mut stdout, "\n🔥 FireChain CLI").unwrap();

    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
    writeln!(
        &mut stdout,
        "🧬 Segurança blockchain com modularidade, criptografia e elegância CLI-first\n"
    )
    .unwrap();

    let _ = stdout.reset();
}

/// ℹ️ Imprime ajuda básica customizada (UX premium)
pub fn print_help() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true));
    writeln!(&mut stdout, "\n📖 Comandos disponíveis:\n").unwrap();

    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    writeln!(&mut stdout, "🔐 new   → Gera uma nova carteira FireChain").unwrap();
    writeln!(&mut stdout, "ℹ️  help  → Exibe ajuda detalhada\n").unwrap();

    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    writeln!(
        &mut stdout,
        "Exemplo:\n  $ firechain-cli new\n"
    )
    .unwrap();

    let _ = stdout.reset();
}
