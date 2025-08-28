use piston_window::color::WHITE;
use piston_window::types::Color;
use piston_window::*;

use crate::draw::{draw_rectangle, draw_text};

const ITEM_SELECTED_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const VALUE_COLOR: Color = [0.60, 0.90, 0.00, 1.0];
pub enum Taille {
    Petit,
    Moyen,
    Grand,
}

impl Taille {
    pub fn get_taille(&self) -> i32 {
        match *self {
            Taille::Petit => 30,
            Taille::Moyen => 50,
            Taille::Grand => 70,
        }
    }

    pub fn get_texte(&self) -> String {
        match *self {
            Taille::Petit => "Petit".to_owned(),
            Taille::Moyen => "Moyen".to_owned(),
            Taille::Grand => "Grand".to_owned(),
        }
    }

    pub fn augmente(&self) -> Taille {
        match *self {
            Taille::Grand => Taille::Petit,
            Taille::Moyen => Taille::Grand,
            Taille::Petit => Taille::Moyen,
        }
    }

    pub fn diminue(&self) -> Taille {
        match *self {
            Taille::Grand => Taille::Moyen,
            Taille::Moyen => Taille::Petit,
            Taille::Petit => Taille::Grand,
        }
    }
}

pub enum Vitesse {
    Rapide,
    Normal,
    Lent,
}

impl Vitesse {
    pub fn get_vitesse(&self) -> f64 {
        match *self {
            Vitesse::Rapide => 0.05,
            Vitesse::Normal => 0.1,
            Vitesse::Lent => 0.2,
        }
    }

    pub fn get_texte(&self) -> String {
        match *self {
            Vitesse::Rapide => "Rapide".to_owned(),
            Vitesse::Normal => "Normal".to_owned(),
            Vitesse::Lent => "Lent".to_owned(),
        }
    }

    pub fn augmente(&self) -> Vitesse {
        match *self {
            Vitesse::Rapide => Vitesse::Lent,
            Vitesse::Normal => Vitesse::Rapide,
            Vitesse::Lent => Vitesse::Normal,
        }
    }

    pub fn diminue(&self) -> Vitesse {
        match *self {
            Vitesse::Rapide => Vitesse::Normal,
            Vitesse::Normal => Vitesse::Lent,
            Vitesse::Lent => Vitesse::Rapide,
        }
    }
}

pub enum Niveau {
    Debutant,
    Initie,
    Champion,
    Maitre,
}

impl Niveau {
    pub fn get_niveau(&self) -> i32 {
        match *self {
            Niveau::Debutant => 1,
            Niveau::Initie => 2,
            Niveau::Champion => 3,
            Niveau::Maitre => 4,
        }
    }

    pub fn get_texte(&self) -> String {
        match *self {
            Niveau::Debutant => "Débutant".to_owned(),
            Niveau::Initie => "Initié".to_owned(),
            Niveau::Champion => "Champion".to_owned(),
            Niveau::Maitre => "Maître".to_owned(),
        }
    }

    pub fn augmente(&self) -> Niveau {
        match *self {
            Niveau::Debutant => Niveau::Initie,
            Niveau::Initie => Niveau::Champion,
            Niveau::Champion => Niveau::Maitre,
            Niveau::Maitre => Niveau::Debutant,
        }
    }

    pub fn diminue(&self) -> Niveau {
        match *self {
            Niveau::Debutant => Niveau::Maitre,
            Niveau::Initie => Niveau::Debutant,
            Niveau::Champion => Niveau::Initie,
            Niveau::Maitre => Niveau::Champion,
        }
    }
}

#[derive(Clone)]
pub struct Bouton {
    name: BoutonType,
    texte: String,
    starter: bool,
}

impl Bouton {
    pub fn new(texte: String, name: BoutonType, starter: bool) -> Bouton {
        Bouton {
            texte,
            name,
            starter,
        }
    }

    pub fn start(&self) -> bool {
        self.starter
    }
}

#[derive(Clone)]
pub enum BoutonType {
    Largeur,
    Hauteur,
    Vitesse,
    Niveau,
    Depart,
}
pub struct Menu {
    width: i32,
    height: i32,
    width_selected: Taille,
    height_selected: Taille,
    vitesse_selected: Vitesse,
    niveau_selected: Niveau,
    buttons: [Bouton; 5],
    button_selected: usize,
    is_closed: bool,
}

impl Menu {
    pub fn new(
        width: i32,
        height: i32,
        width_s: Taille,
        height_s: Taille,
        vitesse: Vitesse,
        niveau: Niveau,
        buttons: [Bouton; 5],
        button: usize,
        is_closed: bool,
    ) -> Menu {
        Menu {
            width,
            height,
            width_selected: width_s,
            height_selected: height_s,
            vitesse_selected: vitesse,
            niveau_selected: niveau,
            button_selected: button,
            is_closed: is_closed,
            buttons,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_closed {
            return;
        }

        match key {
            Key::Up => match self.button_selected {
                0 => self.button_selected = 4,
                _ => self.button_selected -= 1,
            },
            Key::Down => match self.button_selected {
                4 => self.button_selected = 0,
                _ => self.button_selected += 1,
            },
            Key::Left => self.gauche(),
            Key::Right => self.droite(),
            Key::Return => self.close_and_start(),
            _ => (),
        };
    }

    fn close_and_start(&mut self) {
        self.is_closed = self.buttons[self.button_selected].start();
    }

    pub fn to_continue(&self) -> bool {
        self.is_closed
    }

    pub fn return_to_menu(&mut self) {
        self.is_closed = false;
    }

    pub fn draw(&self, context: &Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, context, g);

        let ssx = 10;
        let mut ssy = 10;
        for button in self.buttons.iter() {
            if button.texte == self.buttons[self.button_selected].texte {
                draw_rectangle(ITEM_SELECTED_COLOR, ssx, ssy - 3, 15, 5, context, g);
                draw_text(
                    WHITE,
                    ssx + 2,
                    ssy,
                    button.texte.as_str(),
                    20,
                    glyph_cache,
                    context,
                    g,
                );
            } else {
                draw_text(
                    VALUE_COLOR,
                    ssx + 3,
                    ssy,
                    button.texte.as_str(),
                    20,
                    glyph_cache,
                    context,
                    g,
                );
            }
            let texte_to_write: String = match button.name {
                BoutonType::Largeur => self.width_selected.get_texte(),
                BoutonType::Hauteur => self.height_selected.get_texte(),
                BoutonType::Vitesse => self.vitesse_selected.get_texte(),
                BoutonType::Niveau => self.niveau_selected.get_texte(),
                _ => "".to_owned(),
            };
            draw_text(
                WHITE,
                ssx + 20,
                ssy,
                &texte_to_write.as_str(),
                20,
                glyph_cache,
                context,
                g,
            );

            ssy += 5;
        }
    }

    fn gauche(&mut self) {
        match self.buttons[self.button_selected].name {
            BoutonType::Largeur => self.width_selected = self.width_selected.diminue(),
            BoutonType::Hauteur => self.height_selected = self.height_selected.diminue(),
            BoutonType::Vitesse => self.vitesse_selected = self.vitesse_selected.diminue(),
            BoutonType::Niveau => self.niveau_selected = self.niveau_selected.diminue(),
            _ => (),
        }
    }

    fn droite(&mut self) {
        match self.buttons[self.button_selected].name {
            BoutonType::Largeur => self.width_selected = self.width_selected.augmente(),
            BoutonType::Hauteur => self.height_selected = self.height_selected.augmente(),
            BoutonType::Vitesse => self.vitesse_selected = self.vitesse_selected.augmente(),
            BoutonType::Niveau => self.niveau_selected = self.niveau_selected.augmente(),
            _ => (),
        }
    }

    pub fn get_width(&self) -> i32 {
        self.width_selected.get_taille()
    }

    pub fn get_height(&self) -> i32 {
        self.height_selected.get_taille()
    }

    pub fn get_vitesse(&self) -> f64 {
        self.vitesse_selected.get_vitesse()
    }

    pub fn get_niveau(&self) -> i32 {
        self.niveau_selected.get_niveau()
    }
}
