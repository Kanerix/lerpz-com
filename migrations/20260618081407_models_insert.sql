INSERT INTO models (display_name, description, family, deployment_name, provider, settings)
VALUES ('Gemini 2.5 Flash', 'A really fast generative AI model', 'google', '@google/gemini-2.5-flash', 'Google', '{"reasoning": true}');

INSERT INTO models (display_name, description, family, deployment_name, provider, settings)
VALUES ('GPT 5.5', 'A new generation of GPT model', 'openai', '@azure/gpt-5.5', 'Azure', '{"reasoning": true}');

INSERT INTO models (display_name, description, family, deployment_name, provider, settings)
VALUES ('Claude Opus 4.8', 'Cheapest Anthropic model', 'anthropic', '@google-eu/anthropic.claude-opus-4-8', 'Anthropic', '{"reasoning": true}');

INSERT INTO models (display_name, description, family, deployment_name, provider, settings)
VALUES ('GPT Image 2', 'Newest Azure image model', 'openai', '@azure/gpt-image-2', 'Azure', '{"reasoning": false}');
