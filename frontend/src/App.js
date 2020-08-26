import React from 'react';
import logo from './logo.svg';
import './App.css';
import heading from './components/heading/heading';
class App extends React.Component {
constructor() {
    super();
    this.state = {
		listening: "Listen",
		showUpload: false,
		uploadStatus: "Upload"
    };


	this.handleSubmitUpload = this.handleSubmitUpload.bind(this)
	this.handleSubmitListen = this.handleSubmitListen.bind(this);
  }
 handleSubmitUpload(event) {
	let requestOptions = {
	  method: 'GET',
	};
	
	let url="/api/upload" 
	this.setState({"uploadStatus" : "Uploading"});
	fetch(url, requestOptions)
	  .then(response => response.text())
	  .then(() => 	 this.setState({
		  "uploadStatus" : "Uploaded"
	 }))
	  .catch(error => alert('error', error));
	event.preventDefault();
  }


 handleSubmitListen(event) {
	let requestOptions = {
	  method: 'GET',
	};
	
	let url="./api/toggleListen" 
	 let value;
	 if (this.state.listening === "Listening") {
			value = "Stopped Listenning"
	 } else {
		 value = "Listening"
	 }

	fetch(url, requestOptions)
	  .then(() => 	 this.setState({
		 "listening" : value, // Listenning status = Listenning / Stoped Listenning
		  "showUpload": true,
	 }))

	  .catch(error => console.log('error', error));
	event.preventDefault();
  }
    render() { 
	if (this.state.showUpload) {
		return(<div className="formWrap">
			<heading/>
		  <form className="formWrap" onSubmit={this.handleSubmitListen}>
			<div className="formGroup">
				 <input className="btn" id={this.state.listening} type="submit" value={this.state.listening} />
			</div>
		  </form> 

		  <form className="formWrap" onSubmit={this.handleSubmitUpload}>

			<div className="formGroup">
				 <input className="btn" id={this.state.listening? "listening" : null} type="submit" value={this.state.uploadStatus} />
			</div>
		  </form> 
	</div>
	);
	} else {
		return( <div>
			<heading/>
				<div className="formWrap">
		  <form className="formWrap" onSubmit={this.handleSubmitListen}>
			<div className="formGroup">
				 <input className="btn" id={this.state.listening} type="submit" value={this.state.listening} />
			</div>
		  </form>
			</div>
		</div>
		);
	}
  }
  
}

export default App;
