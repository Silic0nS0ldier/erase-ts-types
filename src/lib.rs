use wasm_bindgen::prelude::*;
use swc_common::{sync::Lrc, FileName, SourceMap, Mark, Globals, GLOBALS};
use swc_ecma_ast::EsVersion;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_transforms_typescript::strip::strip;
use swc_ecma_visit::FoldWith;

#[wasm_bindgen]
pub fn erase(source: String) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("stub.ts".into()), source.into());
    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().expect("oh no");

    let mut buf = vec![];
    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        // no ts
        let top_level_mark = Mark::fresh(Mark::root());
        let module = module.fold_with(&mut strip(top_level_mark));
        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config {
                ..Default::default()
            },
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::with_target(cm, "\n", &mut buf, None, EsVersion::latest()),
        };
        emitter.emit_module(&module).unwrap();
    });

    let code = String::from_utf8_lossy(&buf).to_string();

    return code.to_string();
}

#[cfg(test)]
mod tests {
    use crate::erase;

    #[test]
    fn it_works() {
        assert_eq!(erase("export const foo: Bar = 'foobar';".to_string()), "export const foo = 'foobar';\n");
    }
}
