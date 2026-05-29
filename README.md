# rbuild

Since I wanted to get some idea of rust + gtk4
this is port of python pyBuild.
I know this can be done with some lines of bash, 
but i wanted to build a simple Gui-project.
It provides some basic support for an overview of projects
and makes the use of makepkg, repo-add and installation with pacman easy. 
The commands may be adaptable, but sorry at this time you
need to edit src/proj/mod.rs 

The config is placed in ~/.config/pyBuild.conf you probably 
want to adapt this to your local environment 
(some example will be created on the first start). 

## Buidling 

Rust supports two environment maintenance options:
- by distro install rust/rustc (does not work as for debian the rustc version ist not up to some ofthe package requirements, but maybe for a diffrent distro/version)
- by rust itself rustup, preferred for development  

Requirements for Debian e.g. (install as root):
<pre>
apt-get install rustup
apt-get install libgtk-4-dev
</pre>

To build use: 
<pre>
cd rbuild
cargo build
</pre>

## Examples

raskpass ist not exactly a example more a utility to ask for the sudo password.
If you have already such a utility and the environment SUDO_ASKPASS is set,
the Askpass entry will be ignored. If you need no askpass utility leave the entry
in the config file empty:

<pre>
Askpass=
</pre>

The raskpass utility was provided to avoid additional dependencies, 
it can be built & installed with: 

<pre>
cargo install --path . --examples
</pre>

Configure the install location in 
pyBuild.conf with Askpass in section main e.g.:

<pre>
Askpass=/home/USER/.cargo/bin/raskpass
</pre>

A alternative would be the use zenity (check with zenity --help).
As zenity needs a --password parameter it can't be used with sudo directly.
You have to use a helper script (some hassle I wanted to avoid).

## Gtk4

Getting started with gtk4 is not so hard, most things are like
they used to be. But the builder file format has changed, at
first you may decide to just remove the properties the builder
complains about and in some cases it just works.
But then some strange isssues arise, e.g. entry
fields that are not focusable ... and after hours you realize
there is more to it e.g. the boolean properties no longer work
with true or false they use numeric value 1 or 0.
To convert glade gtk3 .ui files use:

<pre>
gtk4-builder-tool simplify --3to4 BUILDER.ui
</pre>

Or use the new UI designer...

## Thoughts

- the compiler is fast
- the provided error handling capabilities are quite diverse
- for me rust solves the missing types from python (I'm probably a old school type of programmer ;)  
- the linter makes helpful suggestions, especially if you just started 
- JetBrains offers a convenient option to work with rust 
- the target directory grows to ~2..4Gbyte fast, if you use gtk4 (clean with cargo clean)
- as this uses limited dependencies it also will work with a 2G Ram Raspi (bring some time) 
