# 0.8
* Update _ttf-parser_ to `0.8` [changelog](https://github.com/RazrFalcon/ttf-parser/blob/master/CHANGELOG.md#080---2020-07-21).
* `OwnedFace::from_vec` now returns a `Result`.

# 0.7
* Update _ttf-parser_ to `0.7` [changelog](https://github.com/RazrFalcon/ttf-parser/blob/master/CHANGELOG.md#070---2020-07-16).
* Update `*Font` -> `*Face` to reflect the _ttf-parser_ API changes. 
  ```rust
  // 0.6
  let owned_font = OwnedFont::from_vec(owned_font_data, 0)?;

  // 0.7
  let owned_face = OwnedFace::from_vec(owned_font_data, 0)?;
  ```

# 0.6
* Update _ttf-parser_ to `0.6`.

# 0.5.1
* Support no_std.

# 0.5
* Implement crate supporting _ttf-parser_ `0.5`.
