(function() {
  const ZYN_DEFINITION = function(hljs) {
    const PIPE_ARG = {
      className: 'params',
      begin: /:/,
      end: /(?=[:|]|\}\})/,
      contains: [hljs.QUOTE_STRING_MODE, hljs.NUMBER_MODE]
    };

    const PIPE = {
      className: 'meta',
      begin: /\|/,
      end: /(?=\}\})/,
      contains: [
        { className: 'built_in',       begin: /\b(?:upper|lower|snake|camel|pascal|kebab|screaming|ident|fmt|str|trim|plural|singular)\b/ },
        { className: 'title.function', begin: /\b[a-z_][a-z0-9_]*\b/ },
        PIPE_ARG,
      ]
    };

    const INTERPOLATION = {
      className: 'template-variable',
      begin: /\{\{/,
      end: /\}\}/,
      contains: [
        PIPE,
        { subLanguage: 'rust', begin: /[^\s|}]/, end: /(?=[|}])/, excludeEnd: true },
      ]
    };

    const DIRECTIVE = {
      className: 'keyword',
      begin: /@(?:else(?:[ \t]+if)?|if|for|match)\b/
    };

    const CUSTOM_ELEMENT = {
      className: 'title.function',
      begin: /@[a-z_][a-z0-9_]*(?:::[a-z_][a-z0-9_]*)*/
    };

    const PROP = {
      begin: /\b[a-z_][a-z0-9_]*\b(?=\s*=)/,
      returnBegin: true,
      contains: [
        { className: 'attr',     begin: /[a-z_][a-z0-9_]*/ },
        { className: 'operator', begin: /=/ },
      ]
    };

    const RUST_ATTR = {
      className: 'meta',
      begin: /#!?\[/,
      end: /\]/,
      contains: [hljs.QUOTE_STRING_MODE]
    };

    const rust = hljs.getLanguage('rust');
    const rustKeywords = (rust && rust.keywords) ? rust.keywords : {};

    return {
      name: 'zyn',
      aliases: ['zyn'],
      keywords: {
        ...rustKeywords,
        keyword: (rustKeywords.keyword || ''),
      },
      contains: [
        DIRECTIVE,
        CUSTOM_ELEMENT,
        INTERPOLATION,
        PROP,
        hljs.C_LINE_COMMENT_MODE,
        hljs.C_BLOCK_COMMENT_MODE,
        hljs.QUOTE_STRING_MODE,
        hljs.NUMBER_MODE,
        RUST_ATTR,
      ],
    };
  };

  const initZyn = () => {
    if (typeof hljs === 'undefined') return;
    hljs.registerLanguage('zyn', ZYN_DEFINITION);
    const highlightFn = hljs.highlightElement || hljs.highlightBlock;
    document.querySelectorAll('code.language-zyn, code.language-rust').forEach((block) => {
      if (block.textContent.includes('@') || block.textContent.includes('{{')) {
        block.classList.add('language-zyn');
        highlightFn(block);
      }
    });
  };

  if (document.readyState === 'loading') {
    window.addEventListener('load', initZyn);
  } else {
    initZyn();
  }
})();
