use bevy::prelude::*;

use crate::{components::{TexteEnergie, TexteMinerai, TexteVitesse, VitesseGlobale}, update_text_vitesse};

/*
 *** Fonction pour augmenter ou diminuer la vitesse globale de l'application et du parcours des robots
 */ 
pub fn toggle_vitesse(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut vitesse_globale: ResMut<VitesseGlobale>,
    mut query_vitesse: Query<&mut Text, (With<TexteVitesse>, Without<TexteEnergie>, Without<TexteMinerai>)>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) && vitesse_globale.vitesse < 150 {
        vitesse_globale.vitesse += 1;
        update_text_vitesse(&mut query_vitesse, vitesse_globale)
    }
    else if keyboard_input.just_pressed(KeyCode::F1) && vitesse_globale.vitesse > 1 {
        vitesse_globale.vitesse -= 1;
        update_text_vitesse(&mut query_vitesse, vitesse_globale)
    }
}