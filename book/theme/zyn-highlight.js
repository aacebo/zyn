hljs.registerLanguage('zyn', function(hljs) {
    return {
        name: 'Zyn',
        subLanguage: 'rust',
        contains: [
            {
                begin: /\{\s*\{/,
                end: /\}\s*\}/,
                beginScope: 'template-tag',
                endScope: 'template-tag',
                subLanguage: 'rust',
                contains: [
                    {
                        begin: [/\|/, /\s*/, /[a-z][a-z0-9_]*/, /\s*/, /:/, /"[^"]*"/],
                        beginScope: {
                            1: 'keyword',
                            3: 'title.function',
                            5: 'keyword',
                            6: 'string',
                        },
                    },
                    {
                        begin: [/\|/, /\s*/, /[a-z][a-z0-9_]*/],
                        beginScope: {
                            1: 'keyword',
                            3: 'title.function',
                        },
                    },
                ],
            },
            {
                scope: 'keyword',
                match: /@(?:if|else\s+if|else|for|match|throw)\b/,
            },
            {
                begin: [/@/, /[a-z_][a-z0-9_]*(?:::[a-z][a-z0-9_]*)*/],
                beginScope: {
                    1: 'keyword',
                    2: 'title.function',
                },
            },
        ],
    };
});

document.querySelectorAll('code.language-zyn').forEach(function(el) {
    delete el.dataset.highlighted;
    hljs.highlightElement(el);
});
