LIBRW_VENDOR := vendor/librw

.DEFAULT_GOAL := help

.PHONY: help vendor-init vendor-sync vendor-status vendor-diff vendor-update

help:
	@printf '%s\n' \
		'vendor commands:' \
		'  make vendor-init    initialize librw at the pinned rev' \
		'  make vendor-sync    sync submodule URLs and check out the pinned rev' \
		'  make vendor-status  show the pinned and checked-out librw revs' \
		'  make vendor-diff    show changes to the pin and inside the submodule' \
		'  make vendor-update  advance librw to the upstream remote rev'

vendor-init:
	git submodule update --init --recursive -- $(LIBRW_VENDOR)

vendor-sync:
	git submodule sync --recursive -- $(LIBRW_VENDOR)
	git submodule update --init --recursive -- $(LIBRW_VENDOR)

vendor-status:
	git submodule status --recursive -- $(LIBRW_VENDOR)
	@if [ -e "$(LIBRW_VENDOR)/.git" ]; then \
		git -C $(LIBRW_VENDOR) status --short --branch; \
	fi

vendor-diff:
	git diff --submodule=log -- $(LIBRW_VENDOR)
	@if [ -e "$(LIBRW_VENDOR)/.git" ]; then \
		git -C $(LIBRW_VENDOR) diff --stat; \
		git -C $(LIBRW_VENDOR) diff; \
	fi

vendor-update: vendor-init
	git submodule update --remote --recursive -- $(LIBRW_VENDOR)
	@printf '%s\n' 'review and commit the updated $(LIBRW_VENDOR) rev'
