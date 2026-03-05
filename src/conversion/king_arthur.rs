use crate::conversion::data::IngredientWeight;
use scraper::{Html, Selector};

#[allow(dead_code)]
pub async fn fetch_king_arthur_weights() -> Result<Vec<IngredientWeight>, Box<dyn std::error::Error>>
{
    let url = "https://www.kingarthurbaking.com/learn/ingredient-weight-chart";
    let resp = reqwest::get(url).await?.text().await?;
    Ok(parse_king_arthur_html(&resp))
}

#[allow(dead_code)]
pub fn parse_king_arthur_html(html: &str) -> Vec<IngredientWeight> {
    let fragment = Html::parse_document(html);
    let row_selector = Selector::parse("table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut weights = Vec::new();

    for row in fragment.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() >= 4 {
            let name = cells[0].text().collect::<String>().trim().to_string();
            let volume = cells[1].text().collect::<String>().trim().to_string();
            let grams_str = cells[3].text().collect::<String>().trim().to_string();

            // We only care about "1 cup" or similar base measurements for now
            if volume.to_lowercase().contains("cup")
                && !volume.contains("/")
                && !volume.starts_with("0")
            {
                // Parse grams (e.g., "120g" -> 120.0)
                if let Some(grams) = parse_grams(&grams_str) {
                    weights.push(IngredientWeight {
                        name,
                        grams_per_cup: grams,
                    });
                }
            }
        }
    }

    weights
}

#[allow(dead_code)]
fn parse_grams(s: &str) -> Option<f64> {
    // Extract numeric part from "120g" or "120"
    let numeric: String = s
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    numeric.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mock_html() {
        let html = r#"
            <table>
                <tbody>
                    <tr>
                        <td>All-Purpose Flour</td>
                        <td>1 cup</td>
                        <td>4 1/4 oz</td>
                        <td>120g</td>
                    </tr>
                    <tr>
                        <td>Sugar</td>
                        <td>1 cup</td>
                        <td>7 oz</td>
                        <td>198g</td>
                    </tr>
                </tbody>
            </table>
        "#;
        let weights = parse_king_arthur_html(html);
        assert_eq!(weights.len(), 2);
        assert_eq!(weights[0].name, "All-Purpose Flour");
        assert_eq!(weights[0].grams_per_cup, 120.0);
        assert_eq!(weights[1].name, "Sugar");
        assert_eq!(weights[1].grams_per_cup, 198.0);
    }
}
