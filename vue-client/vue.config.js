module.exports = {
  chainWebpack: config => {
    config.module.rule('shader-loader')
      .test("/\.(glsl|vs|fs)$/")
      .use('shader-loader')
      .loader('shader-loader')
      .options({
        // glsl: { chunkPath: resolve("/glsl/chunks") }
      })
      .end()
    config.resolve.modules.add('./src')
  }
}
