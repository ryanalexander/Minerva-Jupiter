# Minerva Backend

This is the primary repo for the Minerva backend.
I will eventually make this a monorepo with all backend services, such as the scraper and search microservice

| Service          | Path                                                              | Description                                   |
| ---------------- | ----------------------------------------------------------------- | --------------------------------------------- |
| Argo             | [/MinervaSearch/Argo](https://github.com/MinervaSearch/Argus)     | Primary web scraper for obtaining web-data    |
| Jupiter          | [/MinervaSearch/Jupiter](https://github.com/MinervaSearch/Jupiter)| Primary rest api for all requests             |

Note: `search` will not be included in this until it is fully functional

## Contributing

Please ensure you are only commiting fully implemented features, and utilising the [Conventional Commits format](https://www.conventionalcommits.org/en/v1.0.0/)

## License

The Minerva backend is generally licensed under the [GNU Affero General Public License v3.0](https://github.com/revoltchat/backend/blob/master/LICENSE). Please check individual crates for further license information.