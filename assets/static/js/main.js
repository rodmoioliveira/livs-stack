const {
  operators: { debounce },
  interval,
  fromEvent,
} = rxjs;

const INNER_WIDTH = 1200;

const details = [...document.querySelectorAll('.filters__details')];
const openDetails = (e) => e.setAttribute('open', true);
const closeDetails = (e) => e.removeAttribute('open');
const isMobile = () => window.innerWidth <= INNER_WIDTH;
const handleDetails = () => {
  if (isMobile()) {
    details.forEach(closeDetails);
  } else {
    details.forEach(openDetails);
  }
};

fromEvent(window, 'resize')
  .pipe(debounce(() => interval(300)))
  .subscribe(handleDetails);
