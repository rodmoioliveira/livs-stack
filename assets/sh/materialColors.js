// Paste this code on the console of
// https://material.io/resources/color/
const materialUiColors = [
  ...new Set(
    [...document.querySelectorAll('.color')]
      .filter(
        (a) =>
          a.getAttribute('aria-label').indexOf('600') < 0 &&
          a.getAttribute('aria-label').indexOf('700') < 0 &&
          a.getAttribute('aria-label').indexOf('800') < 0 &&
          a.getAttribute('aria-label').indexOf('900') < 0 &&
          a.getAttribute('aria-label').indexOf('Brown') &&
          a.getAttribute('aria-label').indexOf('A200') &&
          a.getAttribute('aria-label').indexOf('A300') &&
          a.getAttribute('aria-label').indexOf('A400')
      )
      .map((e) =>
        e.getAttribute('value').replace('#', '').toLowerCase()
      )
  ),
];
copy(materialUiColors);
