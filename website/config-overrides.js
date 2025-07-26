const { addWebpackPlugin, addBabelPlugin, override, overrideDevServer } = require('customize-cra');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const addCustomHeaders = () => (config) => {
  config.headers = {
    'Cross-Origin-Embedder-Policy': 'require-corp',
    'Cross-Origin-Opener-Policy': 'same-origin',
  };
  return config;
};

module.exports = {
  webpack: override(
    addWebpackPlugin(
      new CopyWebpackPlugin({
        patterns: [{ from: '../dist/pkg/cozy.d.ts', to: 'src/types/ultimate.d.ts' }],
      })
    ),
    addBabelPlugin([
      '@emotion',
      {
        // Optional plugin config:
        // sourceMap: true,
        // autoLabel: 'dev-only',
        // labelFormat: '[filename]--[local]',
        // cssPropOptimization: true,
      },
    ])
  ),
  devServer: overrideDevServer(addCustomHeaders()),
};
