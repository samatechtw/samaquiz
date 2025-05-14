export default {
  root: true,
  extends: 'stylelint-config-recommended-vue',
  rules: {
    'at-rule-empty-line-before': null,
    'no-empty-source': null,
    'declaration-property-value-no-unknown': [
      true,
      {
        ignoreProperties: {
          '/.*/': '/(v-bind\\(.+\\))|(.*\\$.*)/',
        },
      },
    ],
    'at-rule-no-unknown': [
      true,
      {
        ignoreAtRules: [
          'function',
          'if',
          'else',
          'return',
          'each',
          'include',
          'mixin',
          'define-mixin',
        ],
      },
    ],
    'declaration-empty-line-before': null,
    'rule-empty-line-before': null,
  },
}
