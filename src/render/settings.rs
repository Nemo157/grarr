use settings::{ self, Theme };

pub fn Settings(settings: &settings::Settings) -> ::maud::Markup {
  html! {
    div.block {
      div.block-header {
        h3 { "Settings" }
      }
      div.block-details {
        form method="POST" {
          label for="theme" { "Theme" }
          select id="theme" name="theme" required? {
            option value=(Theme::SolarizedDark) selected?[settings.theme == Theme::SolarizedDark] { "Solarized Dark" }
            option value=(Theme::SolarizedLight) selected?[settings.theme == Theme::SolarizedLight] { "Solarized Light" }
          }
          button type="submit" {
            "Submit"
          }
        }
      }
    }
  }
}
