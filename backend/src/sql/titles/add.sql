INSERT INTO
  titles(
     isbn
    ,author
    ,edition
    ,format
    ,language
    ,genre
    ,pages
    ,publisher
    ,summary
    ,title
    ,year
  )
VALUES
  (
    $1
   ,$2
   ,$3
   ,$4
   ,$5
   ,$6
   ,$7
   ,$8
   ,$9
   ,$10
   ,$11
  ) RETURNING $table_fields;
