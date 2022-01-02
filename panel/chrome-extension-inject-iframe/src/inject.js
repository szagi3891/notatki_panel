// chrome.extension.sendMessage({}, function(response) {
(() => {
	const readyStateCheckInterval = setInterval(function() {

        if (document.readyState === "complete") {
            clearInterval(readyStateCheckInterval);

            // ----------------------------------------------------------
            // This part of the script triggers when page is done loading
            console.log("Instalacja api w ramce", window.location.toString());
            // ----------------------------------------------------------

            document.head.setAttribute('data-iframe-init', '4');

            window.addEventListener('message', (e) => {
                console.info('Iframe - otrzymano wiadomość 0');
                console.info(`Iframe - otrzymano wiadomość ===> ${window.location.toString()}`, e.data);

                if (e.data === 'command_get_html') {
                    const body = window.document.body.innerHTML;


                    window.parent.postMessage(JSON.stringify({
                        type: 'body',
                        body: body
                    }), '*');

                    console.info("wysyłam post message do parenta");
                }
            });


        }
    }, 10);
})();

      /*
      window.addEventListener('message', (e) => {
          console.info("Informacja od dziecka - otrzymano wiadomość ===> ", e);
      });

      document.querySelectorAll('iframe[src]')[0].contentWindow.postMessage('command_get_html', '*')


      (() => {
      for (const item of document.querySelectorAll('iframe[src]')) {
        const src = item.getAttribute('src');
        console.info('ramka', src, item);
      }
      })()
      */

      // var checkChange = function(){
      //   var flag = false;
      //   var names = document.querySelectorAll('.item-name-container');
      //   var forEach = Array.prototype.forEach;
      //   forEach.call(names, function(nameItem){
      //     if(nameItem.getAttribute('name') === 'test') {
      //       flag = true;
      //     }
      //   });
      //   window.parent.postMessage(flag + '', '*');
      //   console.log('flag: ', flag);
      // };



      // // select the target node
      // var target = document.querySelector('body');

      // // create an observer instance
      // var observer = new MutationObserver(function(mutations) {
      //   mutations.forEach(function(mutation) {
      //     // console.log(mutation.type);
      //     checkChange();
      //   });
      // });

      // // configuration of the observer:
      // var config = { attributes: true, childList: true, characterData: true };

      // // pass in the target node, as well as the observer options
      // observer.observe(target, config);

      // // later, you can stop observing
      // // observer.disconnect();

// });
