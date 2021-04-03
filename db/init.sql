CREATE TABLE IF NOT EXISTS titles (
  isbn BIGSERIAL NOT NULL PRIMARY KEY,
  author VARCHAR(255) NOT NULL,
  title VARCHAR(255) NOT NULL,
  editor VARCHAR(255) NOT NULL,
  description TEXT NOT NULL
);

INSERT INTO
  titles(isbn, author, title, editor, description)
VALUES
  (
    '9788525060600',
    'Elena Ferrante',
    'A amiga genial',
    'Biblioteca Azul - Globo',
    'Quando uma amiga decide desaparecer, a outra resolve contar sua história. A Amiga Genial é o convite para a descoberta do universo de Elena Ferrante- árido, tenso, delicado, profundo e, sobretudo, humano. Duas meninas, no subúrbio de Nápoles passam juntas pelas descobertas da infância, atravessam as turbulências da adolescência e encaram as primeiras convenções da vida adulta.'
  );
