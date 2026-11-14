CLANG_FORMAT ?= clang-format
.DEFAULT_GOAL := help

ROOT_DIR := .
SOURCE_EXTENSIONS := \
	cpp \
	h
EXCLUDED_DIRS := \
	./.git \
	./out \
	./libs

ifeq ($(OS),Windows_NT)
    RUN   := scripts\run.bat
    BUILD := scripts\build.bat
    INIT  := scripts\init_modules.bat
else
    RUN   := bash scripts/run.sh
    BUILD := bash scripts/build.sh
    INIT  := bash scripts/init_modules.sh
endif

.PHONY: help run build build-no-vcpkg clean list-format-files format


help:
	@echo VSNA - available make targets
	@echo   make run               -- start app on stable toolchain
	@echo   make build             -- build project to executable file
	@echo   make build-no-vcpkg    -- build project without vpckg via lib-devel
	@echo   make clean             -- clean out dir before building
	@echo   make format            -- format code files via clang-format
	@echo Scripts live in scripts/ - .bat on Windows, .sh elsewhere.

run:
	$(RUN)

build:
	$(BUILD)

build-no-vcpkg:
	$(BUILD) --no-vcpkg

clean:
	$(BUILD) --clean

list-format-files:
	@find "$(ROOT_DIR)" -type f \
		\( \
			$(foreach ext,$(SOURCE_EXTENSIONS), -name '*.$(ext)' -o) \
			-name '*.cpp' \
		\) \
		$(foreach dir,$(EXCLUDED_DIRS), -not -path '$(dir)' -not -path '$(dir)/*') \
		-print

format:
	@files="$$( $(MAKE) --no-print-directory list-format-files )"; \
	if [ -z "$$files" ]; then \
		echo "Нет файлов для форматирования"; \
	else \
		echo "Форматирование файлов..."; \
		echo "$$files"; \
		${CLANG_FORMAT} -i $$files;\
	fi