#!/bin/bash

case $1 in
    ''|*[!0-9]*)
        echo first argument must be a number
        exit 128
esac

echo inserting rows

for (( i=1; i<=$1; i++ ))
do
    sqlite3 my_db.sqlite < ./scripts/insert.sql
done

echo "inserted $1 row(s)"