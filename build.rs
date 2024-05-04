// Cette directive indique que le crate `winres` doit être utilisé dans ce fichier.
// `winres` est une bibliothèque qui facilite la gestion des ressources Windows dans des projets Rust.
extern crate winres;

// La fonction `main` est le point d'entrée de ce script de build.
fn main() {
    // Ce bloc conditionnel vérifie si le système cible est Windows.
    // `cfg!(target_os = "windows")` évalue à `true` si le code est compilé pour Windows.
    if cfg!(target_os = "windows") {
        // Crée une nouvelle instance de la structure `WindowsResource` du crate `winres`.
        let mut res = winres::WindowsResource::new();

        // Définit l'icône de l'application en spécifiant le chemin vers le fichier `.ico`.
        // Le chemin "assets/icon.ico" doit correspondre à l'emplacement du fichier icône dans votre projet.
        res.set_icon("icon.ico");

        // Compile les ressources définies et les ajoute au projet final.
        // La méthode `compile()` renvoie un `Result`, qui est `Ok` si la compilation des ressources réussit,
        // ou `Err` si une erreur survient.
        if let Err(e) = res.compile() {
            // Si une erreur survient pendant la compilation des ressources, elle est capturée et affichée ici.
            // `eprintln!` imprime le message d'erreur sur la sortie d'erreur standard.
            eprintln!("Failed to add icon: {:?}", e);
        }
    }
}
