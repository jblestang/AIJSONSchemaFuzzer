mod schema;
mod validator;
mod tests;
mod error;
mod fuzzer;

use std::env;
use std::path::PathBuf;

fn print_mutations_list() {
    println!("=== Mutations syntaxiques ===");
    for name in crate::fuzzer::mutation_names::SyntaxMutationName::all() {
        println!("  {} - {}", name.to_string(), name.description());
    }
    println!();
    println!("=== Mutations sémantiques ===");
    println!("  Type:");
    println!("    wrong-type - Type incorrect");
    println!("    out-of-range - Valeur hors plage");
    println!("    null-for-non-nullable - Null pour type non-nullable");
    println!();
    println!("  Enum:");
    println!("    not-in-enum - Valeur non dans l'enum");
    println!("    similar-but-different - Chaîne similaire mais différente");
    println!("    empty-string - Chaîne vide");
    println!();
    println!("  Elements (tableaux):");
    println!("    not-an-array - Pas un tableau");
    println!("    single-invalid-element - Un seul élément invalide");
    println!("    mixed-types - Types mixtes (valides et invalides)");
    println!("    all-invalid-elements - Tous les éléments invalides");
    println!("    completely-different-types - Types complètement différents");
    println!("    empty-array - Tableau vide");
    println!();
    println!("  Values (objets avec valeurs uniformes):");
    println!("    not-an-object - Pas un objet");
    println!("    single-invalid-value - Une valeur invalide");
    println!("    multiple-invalid-values - Plusieurs valeurs invalides");
    println!();
    println!("  Properties (objets avec propriétés):");
    println!("    not-an-object-prop - Pas un objet");
    println!("    all-required-missing - Toutes les propriétés requises manquantes");
    println!("    one-required-missing - Une propriété requise manquante");
    println!("    additional-properties - Propriétés supplémentaires");
    println!("    single-invalid-property - Une propriété invalide");
    println!("    all-invalid-properties - Toutes les propriétés invalides");
    println!("    invalid-optional-property - Propriété optionnelle invalide");
    println!("    null-for-non-nullable-prop - Null pour propriété non-nullable");
    println!("    missing-plus-additional - Propriété manquante + supplémentaire");
    println!("    empty-object - Objet vide");
    println!("    null-object - Objet null");
    println!();
    println!("  Discriminator:");
    println!("    not-an-object-disc - Pas un objet");
    println!("    missing-tag - Tag manquant");
    println!("    invalid-tag - Tag invalide");
    println!("    tag-not-string - Tag non-string");
    println!("    invalid-instance - Instance invalide");
    println!();
    println!("  Ref:");
    println!("    invalid-reference - Référence invalide");
    println!("    non-existent-reference - Référence inexistante");
    println!();
    println!("  Empty:");
    println!("    null-for-empty - Null pour empty");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        eprintln!("Commands:");
        eprintln!("  run-tests          - Exécute tous les tests de conformité");
        eprintln!("  validate <schema> <instance> - Valide une instance contre un schéma");
        eprintln!("  analyze-rfc        - Affiche l'analyse des tests requis par la RFC");
        eprintln!("  fuzz <schema>      - Génère des JSON invalides à partir d'un schéma");
        eprintln!("  generate <schema> [output] - Génère un JSON valide selon le schéma");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "run-tests" => {
            tests::runner::run_all_tests();
        }
        "validate" => {
            if args.len() < 4 {
                eprintln!("Usage: {} validate <schema.json> <instance.json>", args[0]);
                std::process::exit(1);
            }
            let schema_path = PathBuf::from(&args[2]);
            let instance_path = PathBuf::from(&args[3]);
            validator::cli::validate_files(&schema_path, &instance_path);
        }
        "analyze-rfc" => {
            tests::rfc_analysis::print_analysis();
        }
        "fuzz" => {
            if args.len() < 3 {
                eprintln!("Usage: {} fuzz <schema.json> [--syntax|--semantic] [--mutation NAME] [--count N] [--output DIR] [--list-mutations]", args[0]);
                eprintln!();
                eprintln!("Options:");
                eprintln!("  --syntax              Génère des JSON syntaxiquement invalides");
                eprintln!("  --semantic            Génère des JSON sémantiquement invalides (défaut)");
                eprintln!("  --mutation NAME       Spécifie une mutation par nom (voir --list-mutations)");
                eprintln!("  --count N             Nombre de cas à générer (défaut: 5)");
                eprintln!("  --output DIR          Répertoire de sortie");
                eprintln!("  --list-mutations      Liste toutes les mutations disponibles");
                std::process::exit(1);
            }
            let schema_path = PathBuf::from(&args[2]);
            
            let mut mutation_type = fuzzer::mutations::MutationType::Semantic;
            let mut mutation_name = None;
            let mut count = 5;
            let mut output_dir = None;
            let mut list_mutations = false;
            
            let mut i = 3;
            while i < args.len() {
                match args[i].as_str() {
                    "--syntax" => {
                        mutation_type = fuzzer::mutations::MutationType::Syntax;
                    }
                    "--semantic" => {
                        mutation_type = fuzzer::mutations::MutationType::Semantic;
                    }
                    "--mutation" => {
                        if i + 1 < args.len() {
                            mutation_name = Some(args[i + 1].clone());
                            i += 1;
                        }
                    }
                    "--count" => {
                        if i + 1 < args.len() {
                            count = args[i + 1].parse().unwrap_or(5);
                            i += 1;
                        }
                    }
                    "--output" => {
                        if i + 1 < args.len() {
                            output_dir = Some(args[i + 1].clone());
                            i += 1;
                        }
                    }
                    "--list-mutations" => {
                        list_mutations = true;
                    }
                    _ => {
                        eprintln!("Option inconnue: {}", args[i]);
                        std::process::exit(1);
                    }
                }
                i += 1;
            }
            
            if list_mutations {
                print_mutations_list();
                std::process::exit(0);
            }
            
            let options = fuzzer::cli::FuzzerOptions {
                count,
                mutation_type,
                mutation_name,
                output_dir,
            };
            
            if let Err(e) = fuzzer::cli::run_fuzzer(&schema_path, options) {
                eprintln!("Erreur: {}", e);
                std::process::exit(1);
            }
        }
        "generate" => {
            if args.len() < 3 {
                eprintln!("Usage: {} generate <schema.json> [output.json]", args[0]);
                std::process::exit(1);
            }
            let schema_path = PathBuf::from(&args[2]);
            let output_path = if args.len() > 3 {
                Some(PathBuf::from(&args[3]))
            } else {
                None
            };
            
            if let Err(e) = fuzzer::cli::generate_valid(&schema_path, output_path.as_deref()) {
                eprintln!("Erreur: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}
