use bevy::prelude::*;

use crate::components::{Compteur, TexteEnergie, TexteMinerai, TexteVitesse, VitesseGlobale};

/***
 * Fonction pour ajouter l'interface
 */
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
)
{
    let font = asset_server.load("polices/RobotoMono.ttf");

    commands.spawn(TextBundle {
        text: Text::from_sections([
            TextSection::new(
                "Énergies: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::GREEN,
                }
            )
        ])
        .with_justify(JustifyText::Left),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    })
    .insert(TexteEnergie);

    commands.spawn(TextBundle {
        text: Text::from_sections([
            TextSection::new(
                "Minerais: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::PURPLE,
                }
            )
        ])
        .with_justify(JustifyText::Left),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0), 
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    })
    .insert(TexteMinerai);  

    commands.spawn(TextBundle {
        text: Text::from_sections([
            TextSection::new(
                "Vitesse: x",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            ),
            TextSection::new(
                "1",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            )
        ])
        .with_justify(JustifyText::Right),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0), 
            right: Val::Px(25.0),
            ..default()
        },
        ..default()
    })
    .insert(TexteVitesse);    
}

/***
 * Fonction pour ajouter une légende
 */
pub fn setup_legend(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let legend_texture_handle = asset_server.load("legende.png");

    if let Some(_window) = windows.get_single_mut().ok() {
        commands.spawn(SpriteBundle {
            texture: legend_texture_handle,
            transform: Transform::from_xyz(-5.0, 30.0, 0.0).with_scale(Vec3::splat(0.01)),
            ..Default::default()
        });
    }
}

/***
 * Fonction pour mettre à jour le texte des compteurs
 */
pub fn update_text(
    compteur: &Compteur,
    query_energie: &mut Query<&mut Text, (With<TexteEnergie>, Without<TexteMinerai>, Without<TexteVitesse>)>,
    query_minerai: &mut Query<&mut Text, (With<TexteMinerai>, Without<TexteEnergie>, Without<TexteVitesse>)>,
) {
    if let Ok(mut texte_energie) = query_energie.get_single_mut() {
        texte_energie.sections[1].value = compteur.energie.to_string();
    }
    if let Ok(mut texte_minerai) = query_minerai.get_single_mut() {
        texte_minerai.sections[1].value = compteur.minerai.to_string();
    } 
}

/***
 * Fonction pour mettre à jour le texte de la vitesse
 */
pub fn update_text_vitesse(
    query_vitesse: &mut Query<&mut Text, (With<TexteVitesse>, Without<TexteEnergie>, Without<TexteMinerai>)>,
    vitesse_globale: ResMut<VitesseGlobale>
) {
    if let Ok(mut texte_vitesse) = query_vitesse.get_single_mut() {
        texte_vitesse.sections[1].value = vitesse_globale.vitesse.to_string();
    } 
}
