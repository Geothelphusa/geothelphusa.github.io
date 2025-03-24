use stylist::{css, Style};

pub fn responsive_styles() -> Style {
  Style::new(css!(
      r#"
        .container {
            width: 95%; /* 画面幅に合わせてコンテナの幅を調整 */
            max-width: 1200px; /* 最大幅を設定 */
            margin: 0 auto; /* 中央寄せ */
            height: auto; /* 高さを自動調整 */
        }

        @media (min-width: 768px) {
            .container {
                width: 70%;
            }
        }

        @media (min-width: 1200px) {
            .container {
                width: 50%;
            }
        }
        "#
  ))
  .unwrap()
}
          

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
          
          padding-top: 5vh;
          display: flex;
          flex-direction: column;
          justify-content: top;
          text-align: center;
      "#
  ))
  .unwrap()
}


pub fn title_logo() -> Style {
  Style::new(css!(
      r#"
          width: 500px;
          height: 500px;
          max-width: 100%;
          max-height: 100%;

          @media (max-width: 768px) { /* 768px以下（スマホなど）の場合 width/height は 3/4 にする */
              width: 300px;
              height: 300px;
              max-width: 75%;
              max-height: 100%;
          }
      "#
  )).unwrap()
}

pub fn nav_styles() -> Style {
  Style::new(css!(
      r#"
          position: fixed;
          top: 0;
          left: 0;
          width: 100%;
          height: 100px; /* 画面上端まで拡張 */
          background: #2f2f2f;
          display: flex;
          align-items: center;
          justify-content: space-between;
          padding: 0 20px;
          z-index: 100;
      "#
  )).unwrap()
}

pub fn menu_items() -> Style {
  Style::new(css!(
      r#"
          display: block;
          text-decoration: none;
          color: black;
          margin-right: 35px;
      "#
  )).unwrap()
}


pub fn toggle_button() -> Style {
  Style::new(css!(
      r#"   
            position: absolute;
            top: 10px;
            right: 20px;
            z-index: 150;
            
            display: flex;
            width: 56px;
            height: 28px;
            border: 1px solid #555555;
            border-radius: 9999px;
            background-color: #dddddd;
            cursor: pointer;

          :has(:focus-visible) {
            outline: auto;
            outline: auto -webkit-focus-ring-color;
          }

      "#
  )).unwrap()
}

pub fn toggle_slider() -> Style {
  Style::new(css!(
      r#"
          
            appearance: none;
            position: absolute;
            top: 40%;
            left: 0px;
            width: 24px;
            height: 24px;
            border: 1px solid #555555;
            border-radius: 9999px;
            transform: translateY(-50%);
            outline: none;
            background-color: #ffffff;
            transition: left 0.2s;
            cursor: pointer;
          

          :checked {
            left: calc(100% - 32px);
            background-color: #2f2f2f;
          }
      "#
  )).unwrap()
}

pub fn overlay_style() -> Style {
  Style::new(css!(
  r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.8); /* 透明度を少し暗めに */
        display: flex;
        flex-direction: column;
        justify-content: center; /* 中央配置 */
        align-items: center; /* 中央配置 */
        z-index: 200; /* nav より前面に */
        opacity: 0;
        visibility: hidden;
        transition: opacity 0.3s ease, visibility 0.3s ease;

        &.is-opened {
            opacity: 1;
            visibility: visible;
        }
  "#
))
.unwrap()
}

pub fn menu_style() -> Style {
  Style::new(css!(
  r#"
        background: white;
        padding: 40px;
        border-radius: 10px;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
        row-gap: 20px;
        align-items: center;
        width: 80%;
        max-width: 400px;
        text-align: center;
        z-index: 201;

  "#
))
.unwrap()
}


pub fn menu_list_style() -> Style {
  Style::new(css!(
      r#"
      list-style: none;
      padding: 0;
      margin: 0;
      display: flex;
      flex-direction: column;
      align-items: center;
      row-gap: 15px;
      font-size: 20px;
      font-weight: bold;
      color: #333;
      "#
  ))
  .unwrap()
}

pub fn menu_button_style() -> Style {
  Style::new(css!(
      r#"
      height: 45px;
      width: 45px;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      row-gap: 6px;

      &__line,
      &::before,
      &::after {
          content: "";
          width: 28px;
          height: 2px;
          background-color: #333333;
          transition: transform 0.3s, opacity 0.3s;
      }

      &.is-opened &__line {
          opacity: 0;
      }

      &.is-opened::before {
          transform: translateY(8px) rotate(45deg);
      }

      &.is-opened::after {
          transform: translateY(-8px) rotate(-45deg);
      }
      "#
  ))
  .unwrap()
}

pub fn link_card_style() -> Style {
  Style::new(css!(
      r#"
        width: 45%;
        box-sizing: border-box;
        background-color: #fff;
        border: 1px solid gray;
        border-radius: 8px;
        margin: 0;
        padding: 10px;
        .link-card {
            display: grid;
            grid-template-columns: 100px 1fr;
            gap: 10px;
            align-items: center;
            text-decoration: none;
        }
        .link-card-image img {
            width: 200%;
            height: auto;
            object-fit: cover;
            text-align: left;
            border-radius: 8px;
            margin-right: 10px;
        }
        .link-card-text {
            display: flex;
            flex-direction: column;
        }
        .link-card-title {
            font-weight: bold;
            margin: 0;
        }
        .link-card-description {
            margin: 5px 0;
            white-space: nowrap;
        }
        .link-card-domain {
            display: flex;
            align-items: center;
            margin-top: 5px;
        }
        .link-card-domain img {
            width: 16px;
            height: 16px;
            margin-right: 5px;
        }
      "#
  ))
  .unwrap()
}