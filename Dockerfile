FROM marcoieni/release-plz:sha-f76b485

LABEL repository="https://github.com/MarcoIeni/release-plz-action"
LABEL homepage="https://github.com/MarcoIeni/release-plz"

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
