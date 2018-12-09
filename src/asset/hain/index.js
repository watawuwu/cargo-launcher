'use strict';

const exec = require('child_process').exec;

module.exports = (pluginContext) => {
  const toast = pluginContext.toast;
  const clipboard = pluginContext.clipboard;
  const logger = pluginContext.logger;

  function startup() {
    process.env.PATH = "~/.cargo/bin:~/.local/bin:/usr/local/bin:" + process.env.PATH;
  }

  function search(query, res) {
    logger.log('query: ' + query);
    logger.log('res: ' + res);
    exec(`{{name}} "${query}"`, (err, stdout, stderr) => {
      if (err) {
        logger.log(`error: ${err}`);
      }
      logger.log(`stdout: ${stdout}`);

      res.add({
        id: 'ok',
        payload: stdout,
        title: `<b>${stdout}</b>`
      });
    });
  }

  function execute(id, payload) {
    if (id === 'ok') {
      clipboard.writeText(payload).then((result) => {
        toast.enqueue(`Added to clipboard: ${payload}`);
      });
    }
  }

  function renderPreview(id, payload, render) {
    render('<html><body>Something</body></html>');
  }

  return { startup, search, execute, renderPreview };
};
