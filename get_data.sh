YEAR=2023
MONTH="01"
VARIANTS=(antichess atomic chess960 crazyhouse horde kingOfTheHill racingKings threeCheck)

mkdir -p data
for variant in ${VARIANTS[@]}; do
    wget -P data "https://database.lichess.org/$variant/lichess_db_${variant}_rated_$YEAR-$MONTH.pgn.zst"
done
