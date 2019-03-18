module.exports = {
  chainWebpack: config => {
    config.resolve.modules.add('./src')
  }
}
