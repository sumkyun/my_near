near call dev-1684508106444-27785725716650 add_item '{"title":"apple", "description":"amazing"}' --accountId songsk.testnet --deposit 0.3
near call dev-1684508106444-27785725716650 add_item '{"title":"banana", "description":"good"}' --accountId songsk.testnet --deposit 0.1
near call dev-1684508106444-27785725716650 add_item '{"title":"melon", "description":"sweet"}' --accountId songsk.testnet --deposit 0.11
near call dev-1684508106444-27785725716650 add_item '{"title":"orange", "description":"wooo", "src":"링크"}' --accountId songsk.testnet --deposit 0.09
near view dev-1684508106444-27785725716650 get_item '{"idx" : 0}'

near view dev-1684582956578-27867779489882 get_number_of_items

near view dev-1684582956578-27867779489882 get_item '{"item_id" : 2}'

dev-1684582956578-27867779489882