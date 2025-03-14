use stylist::{css, Style};

pub fn center_styles() -> Style {
  Style::new(css!(
      r#"
          justify-content: center;
          text-align: center;
      "#
  ))
  .unwrap()
}

pub fn light_mode_styles() -> Style {
  Style::new(css!(
      r#"
          color: #0f0f0f;
          background-color: #f6f6f6;
          position: fixed;
          width: 100vw;
          height: 100vh;
      "#
  ))
  .unwrap()
}

pub fn dark_mode_styles() -> Style {
  Style::new(css!(
      r#"
          color: #f6f6f6;
          background-color: #2f2f2f;
          position: fixed;
          width: 100vw;
          height: 100vh;
      "#
  ))
  .unwrap()
}

pub fn base_styles() -> Style {
  Style::new(css!(
      r#"
          font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
          font-size: 16px;
          line-height: 24px;
          font-weight: 400;
          
          font-synthesis: none;
          text-rendering: optimizeLegibility;
          -webkit-font-smoothing: antialiased;
          -moz-osx-font-smoothing: grayscale;
          -webkit-text-size-adjust: 100%;
          text-align: center;
      "#
  ))
  .unwrap()
}

pub fn container_styles() -> Style {
  Style::new(css!(
      r#"
          margin: 0;
          
          padding-top: 10vh;
          display: flex;
          flex-direction: column;
          justify-content: center;
          text-align: center;
      "#
  ))
  .unwrap()
}

// pub fn row_styles() -> Style {
//   Style::new(css!(
//       r#"
//           display: flex;
//           justify-content: center;
//       "#
//   ))
//   .unwrap()
// }

pub fn input_and_button() -> Style {
  Style::new(css!(
      r#"
          border-radius: 8px;
          border: 1px solid transparent;
          padding: 0.6em 1.2em;
          font-size: 1em;
          font-weight: 500;
          font-family: inherit;
          color: #0f0f0f;
          background-color: #ffffff;
          transition: border-color 0.25s;
          box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
          outline: none;
      "#
  )).unwrap()
}