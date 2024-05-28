use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::measurements::*;
// use uom::{
//     fmt::DisplayStyle::Abbreviation,
//     si::{
//         f64::{Weight, Volume},
//         Weight::{gram, kilogram, milligram, pound},
//         volume::{cup, liter, milliliter, tablespoon, teaspoon},
//     },
// };

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum RecipeError {
    #[error("Expected a title for the recipe")]
    ExpectedTitle,
    #[error("Encountered `image:` but no subsequent href was provided")]
    ExpectedImageHref,
    #[error("Expected `---ingredients` to indicate the start of the ingredient list")]
    ExpectedIngredientsStart,
    #[error("Expected an ingredient, found an empty string")]
    ExpectedIngredient,
    #[error("Expected `---steps` to indicate the start of the recipe steps")]
    ExpectedStepsStart,
    #[error("Expected {0}, found EOF")]
    UnexpectedEOF(String),
    #[error("{0}")]
    CustomString(String),
}

#[test]
fn fromstr() {
    let base = "8 eggs";
    let mass = Volume::from_str(base);
    println!("{mass:?}");
    println!("{:?}", base.split_once(' ').unwrap().0.parse::<u64>());
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum IngredientQuantity<T = Metric> {
    Weight(Weight<T>),
    Volume(Volume<T>),
}

impl<T> IngredientQuantity<T> {
    pub const fn as_imperial(self) -> IngredientQuantity<Imperial> {
        match self {
            Self::Weight(w) => IngredientQuantity::Weight(w.as_imperial()),
            Self::Volume(v) => IngredientQuantity::Volume(v.as_imperial()),
        }
    }

    pub const fn as_metric(self) -> IngredientQuantity<Metric> {
        match self {
            Self::Weight(w) => IngredientQuantity::Weight(w.as_metric()),
            Self::Volume(v) => IngredientQuantity::Volume(v.as_metric()),
        }
    }
}

impl std::fmt::Display for IngredientQuantity<Metric> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngredientQuantity::Weight(w) => write!(f, "{w}"),
            IngredientQuantity::Volume(v) => write!(f, "{v}"),
        }
    }
}

impl std::fmt::Display for IngredientQuantity<Imperial> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngredientQuantity::Weight(w) => write!(f, "{w}"),
            IngredientQuantity::Volume(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredient<T = Metric> {
    pub ingredient: String,
    pub quantity: Option<IngredientQuantity<T>>,
    // unit: PhantomData<U>
}

impl<T> Ingredient<T> {
    pub fn as_imperial(self) -> Ingredient<Imperial> {
        let ingredient = self.ingredient;
        let quantity = self.quantity.map(|q| q.as_imperial());

        Ingredient {
            ingredient,
            quantity
        }
    }

    pub fn as_metric(self) -> Ingredient<Metric> {
        let ingredient = self.ingredient;
        let quantity = self.quantity.map(|q| q.as_metric());

        Ingredient {
            ingredient,
            quantity
        }
    }
}

impl FromStr for Ingredient {
    type Err = RecipeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RecipeError::ExpectedIngredient;

        if s.is_empty() {
            return Err(ExpectedIngredient);
        }

        let amount_i = s
            .char_indices()
            .filter(|(_, c)| *c == ' ')
            .skip(1)
            .next()
            .map(|(i, _)| i)
            .unwrap_or(s.len());

        let amount = &s[..amount_i].trim_end();

        if let Ok(m) = Weight::from_str(amount) {
            let ingredient = s[amount_i..].trim_start().to_string();
            let quantity = Some(IngredientQuantity::Weight(m));

            return Ok(Self {
                ingredient,
                quantity,
            });
        }

        if let Ok(v) = Volume::from_str(amount) {
            let ingredient = s[amount_i..].trim_start().to_string();
            let quantity = Some(IngredientQuantity::Volume(v));

            return Ok(Self {
                ingredient,
                quantity,
            });
        }

        Ok(Self {
            ingredient: s.to_string(),
            quantity: None,
        })
    }
}

impl std::fmt::Display for Ingredient<Metric> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(q) = &self.quantity {
            write!(f, "{q} ")?;
        }
        write!(f, "{}", self.ingredient)
    }
}

impl std::fmt::Display for Ingredient<Imperial> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(q) = &self.quantity {
            write!(f, "{q} ")?;
        }
        write!(f, "{}", self.ingredient)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredients<T = Metric> {
    pub sections: HashMap<String, Vec<Ingredient<T>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe<T = Metric> {
    pub title: String,
    pub image: Option<Image>,
    pub introduction: Option<String>,
    pub ingredients: Vec<Ingredient<T>>,
    pub steps: Vec<Step>,
}

impl Recipe<Metric> {
    pub fn as_imperial(self) -> Recipe<Imperial> {
        let Recipe { title, image, introduction, ingredients, steps } = self;
        let ingredients = ingredients
            .into_iter()
            .map(|i| i.as_imperial())
            .collect::<Vec<_>>();

        Recipe {
            title,
            image,
            introduction,
            ingredients,
            steps
        }
    }
}

impl Recipe<Imperial> {
    pub fn as_metric(self) -> Recipe<Metric> {
        let Recipe { title, image, introduction, ingredients, steps } = self;
        let ingredients = ingredients
            .into_iter()
            .map(|i| i.as_metric())
            .collect::<Vec<_>>();

        Recipe {
            title,
            image,
            introduction,
            ingredients,
            steps
        }
    }
}

impl FromStr for Recipe {
    type Err = RecipeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RecipeError::*;

        let title_end = s.find("\n\n").ok_or(ExpectedTitle)?;
        let title = s[..title_end].to_string();

        println!("{title}");

        let s = s[title_end..].trim_start();

        let (image, s) = if s.starts_with("image:") {
            let image_href_end = s.find("\n\n").ok_or(ExpectedImageHref)?;
            (
                Some(Image {
                    href: s[6..image_href_end].trim_start().to_string(),
                }),
                s[image_href_end..].trim_start(),
            )
        } else {
            (None, s)
        };

        println!("{image:?}");

        let (introduction, s) = if !s.starts_with("---ingredients") {
            let introduction_end = s.find("\n\n").ok_or(ExpectedImageHref)?;
            (
                Some(s[..introduction_end].trim_start().to_string()),
                s[introduction_end..].trim_start(),
            )
        } else {
            (None, s)
        };

        println!("{introduction:?}");

        if !s.starts_with("---ingredients") {
            return Err(ExpectedIngredientsStart);
        }

        let mut s = s[14..].trim_start();
        let mut ingredients = Vec::new();
        while !s.starts_with('\n') {
            let ingredient_end = s.find('\n').ok_or(UnexpectedEOF("Ingredient".into()))?;
            let ingredient = s[..ingredient_end].parse::<Ingredient>()?;
            ingredients.push(ingredient);
            s = &s[(ingredient_end + 1)..];
        }
        println!("{ingredients:?}");

        let s = s.trim_start();

        if !s.starts_with("---steps") {
            return Err(ExpectedStepsStart);
        }
        let s = s[8..].trim();

        let steps = s
            .split("\n\n")
            .map(|s| Step {
                body: s.to_string(),
            })
            .collect::<Vec<_>>();

        println!("{steps:?}");

        Ok(Self {
            title,
            image,
            introduction,
            ingredients,
            steps,
        })
    }
}

#[test]
fn measurements() {
    // let mut m = Weight::new::<gram>(1040.0);
    // m *= 10.1;
    // m = m.round::<pound>();
    // m /= 10.0;

    // println!(
    //     "{}",
    //     m.into_format_args(pound, uom::fmt::DisplayStyle::Abbreviation)
    // );
}
