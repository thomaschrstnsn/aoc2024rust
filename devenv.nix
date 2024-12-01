{ pkgs, lib, config, inputs, ... }:

{
  packages = with pkgs; [ figlet lolcat ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.rust.channel = "stable";

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = ''
    figlet -f univers advent of code | lolcat
  '';
}
