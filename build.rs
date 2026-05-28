// rbuild
fn main() {
  
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/rbuild.gresource.xml",
        "rbuild.gresource",
    );
    
    glib_build_tools::compile_resources(
        &["examples/raskpass/resources"],
        "examples/raskpass/resources/rask.gresource.xml",
        "rask.gresource",
    );
}