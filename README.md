# Stellar_project
A stellar Blockchain project on "Self Publishing Platform for Writers"

# Self Publishing Platform for Writers

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)

## Project Title
Self Publishing Platform for Writers

## Project Description
The Self Publishing Platform for Writers is a decentralized application (dApp) that allows authors to publish their works directly to readers without the need for traditional publishing houses. This platform empowers writers to create, manage, and distribute their content while retaining full control over their intellectual property. The smart contract ensures transparency, security, and a fair revenue-sharing model for all participants.

## Project Vision
To create an accessible and user-friendly platform that democratizes the publishing process, enabling writers to reach their audience directly and efficiently while ensuring fair compensation for their work.

## Key Features
- **Content Creation**: Writers can create and publish their works directly on the platform.
- **Royalty Management**: The smart contract automatically manages royalties, ensuring that writers receive fair compensation for their work.
- **Decentralized Storage**: All published works are stored securely on the blockchain, ensuring permanence and accessibility.
- **Reader Engagement**: Readers can interact with authors, leave feedback, and support their favorite writers through direct contributions.

## Future Scope
- **Enhanced Analytics**: Implement analytics tools for writers to track their readership and engagement metrics.
- **Community Features**: Introduce community features such as forums and discussion boards for writers and readers to interact.
- **Multi-Language Support**: Expand the platform to support multiple languages, allowing writers from different regions to publish their works.
- **Integration with Other dApps**: Explore partnerships with other decentralized applications to enhance the platform's functionality and reach.

## Smart Contract Overview
The smart contract for the Self Publishing Platform includes the following key functions:
- **publish_work**: Allows writers to publish their works by providing a title, author name, and content.
- **view_work**: Enables users to retrieve the details of a published work using its unique ID.
- **total_published_works**: Returns the total number of works published on the platform.

This contract is built using the Soroban SDK and is designed to be efficient, secure, and user-friendly


CONTRACT ID:-CAELCI7NWXTGEFCDDZB7RBHKKVYEDLAEANM52UCN4NJ23VIPR6KHPULL

let image_url = Some(String::from_str(&env, "C:\Users\HP\Desktop\Stellar Project\Self-publishing-platform-for-writers"));
SelfPublishingContract::publish_work(env, title, author, content, image_url);
