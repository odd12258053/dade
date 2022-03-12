/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  sidebar: [
    {
      type: 'doc',
      label: 'Introduction',
      id: 'index',
    },
    {
      type: 'category',
      label: 'Usage',
      items: [
        {
          type: 'doc',
          label: 'Models',
          id: 'usage/models',
        },
        {
          type: 'doc',
          label: 'Fields',
          id: 'usage/fields',
        },
        {
          type: 'doc',
          label: 'Schema',
          id: 'usage/schema',
        },
      ],
    },
    {
      type: 'doc',
      label: 'API',
      id: 'api',
    },
  ]
};

module.exports = sidebars;
