(this.webpackJsonpweb=this.webpackJsonpweb||[]).push([[0],[,,,,,,,,function(t,e,n){t.exports=n(16)},,,,,function(t,e,n){},function(t,e,n){t.exports=n.p+"static/media/logo.5d5d9eef.svg"},function(t,e,n){},function(t,e,n){"use strict";n.r(e);var a=n(0),i=n.n(a),s=n(3),u=n.n(s),l=(n(13),n(4)),r=n(5),o=n(1),c=n(6),h=n(7);n(14),n(15);var m=function(t){return t.shonw?i.a.createElement("form",{className:"formWrap flat",onSubmit:t.submit},i.a.createElement("input",{className:"btn flat",id:t.listening?"listening":null,type:"submit",value:t.value})):null},d=function(t){return i.a.createElement("form",{className:"formWrap",onSubmit:t.submit},i.a.createElement("input",{className:"btn round",id:t.listening,type:"submit",value:t.listening}))},p=function(t){Object(h.a)(n,t);var e=Object(c.a)(n);function n(){var t;return Object(l.a)(this,n),(t=e.call(this)).state={listening:"Listen",uploadStatus:"Upload",dumpStatus:"Dump",clearStatus:"Clear",showUpload:!1,showDump:!1,showClear:!1},t.handleSubmitListen=t.handleSubmitListen.bind(Object(o.a)(t)),t.handleSubmitUpload=t.handleSubmitUpload.bind(Object(o.a)(t)),t.handleSubmitDump=t.handleSubmitDump.bind(Object(o.a)(t)),t.handleSubmitClear=t.handleSubmitClear.bind(Object(o.a)(t)),t}return Object(r.a)(n,[{key:"handleSubmitListen",value:function(t){var e,n=this;"Listening"===this.state.listening?(e="Stopped",this.setState({showUpload:!0})):(e="Listening",this.setState({showUpload:!0,showDump:!1,showClear:!1})),fetch("./api/toggleListen",{method:"GET"}).then((function(){return n.setState({listening:e})})).catch((function(t){return console.log("error",t)})),t.preventDefault()}},{key:"handleSubmitUpload",value:function(t){var e=this;this.setState({uploadStatus:"Uploading"}),fetch("/api/upload",{method:"GET"}).then((function(t){return t.text()})).then((function(){return e.setState({uploadStatus:"Uploaded",showDump:!0})})).catch((function(t){return alert("error",t)})),t.preventDefault()}},{key:"handleSubmitDump",value:function(t){var e=this;this.setState({dumpStatus:"Dumping"}),fetch("/api/dump",{method:"GET"}).then((function(t){return t.text()})).then((function(){return e.setState({dumpStatus:"Dumped",showClear:!0})})).catch((function(t){return alert("error",t)})),t.preventDefault()}},{key:"handleSubmitClear",value:function(t){var e=this;this.setState({clearStatus:"Cleaning"}),fetch("/api/clear",{method:"GET"}).then((function(t){return t.text()})).then((function(){return e.setState({clearStatus:"Cleaned"})})).catch((function(t){return alert("error",t)})),t.preventDefault()}},{key:"render",value:function(){return i.a.createElement("div",{id:"wrapper"},i.a.createElement("div",{className:"formWrap"},i.a.createElement(d,{listening:this.state.listening,submit:this.handleSubmitListen}),i.a.createElement(m,{listening:this.state.listening,submit:this.handleSubmitUpload,value:this.state.uploadStatus,show:this.state.showUpload}),i.a.createElement(m,{listening:this.state.listening,submit:this.handleSubmitDump,value:this.state.dumpStatus,show:this.state.showDump}),i.a.createElement(m,{listening:this.state.listening,submit:this.handleSubmitClear,value:this.state.clearStatus,show:this.state.showClear})))}}]),n}(i.a.Component);Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));u.a.render(i.a.createElement(i.a.StrictMode,null,i.a.createElement(p,null)),document.getElementById("root")),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then((function(t){t.unregister()})).catch((function(t){console.error(t.message)}))}],[[8,1,2]]]);
//# sourceMappingURL=main.28412884.chunk.js.map