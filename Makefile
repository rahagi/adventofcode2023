CURR_DAY=$(shell ls src | grep day | wc -l)
NEW_DAY=$(shell printf '%02d' $$(($(CURR_DAY) + 1)))

newday:
	bash ./bin/new-day.sh $(NEW_DAY)
	bash ./bin/get-input.sh $(NEW_DAY)

test:
	cargo test 0$(CURR_DAY) -- --nocapture

all:
	newday
