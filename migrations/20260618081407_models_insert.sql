INSERT INTO models (display_name, description, family, deployment_name, provider, modalities, settings)
VALUES ('Gemini 2.5 Flash', 'A really fast generative AI model', 'google', '@google/gemini-2.5-flash', 'Google', '{text}', '{"reasoning": true}');

INSERT INTO models (display_name, description, family, deployment_name, provider, modalities, settings)
VALUES ('GPT 5.5', 'A new generation of GPT model', 'openai', '@azure/gpt-5.5', 'Azure', '{text}', '{"reasoning": true}');

INSERT INTO models (display_name, description, family, deployment_name, provider, modalities, settings)
VALUES ('Claude Opus 4.8', 'Cheapest Anthropic model', 'anthropic', '@google-eu/anthropic.claude-opus-4-8', 'Anthropic', '{text}', '{"reasoning": true}');

INSERT INTO models (display_name, description, family, deployment_name, provider, modalities, settings)
VALUES ('GPT Image 2', 'The newest OpenAI image model', 'openai', '@azure/gpt-image-2', 'Azure', '{image}', '{"reasoning": false}');

INSERT INTO models (display_name, description, family, deployment_name, provider, modalities, settings)
VALUES ('Sora 2', 'The best video model from OpenAI', 'openai', '@azure/sora-2', 'Azure', '{video}', '{"reasoning": false}');
