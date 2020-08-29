import React from 'react';
import logo from './logo.svg';
import './App.css';
import heading from './components/heading/heading';
import Button from './components/buttons/button';
import RoundButton from './components/buttons/roundButton';


class App extends React.Component {
constructor() {
    super();
    this.state = {
		listening: "Listen",
		uploadStatus: "Upload",
		dumpStatus: "Dump",
		clearStatus: "Clear", 

		showUpload: false,
		showDump: false,
		showClear: false


    };

	this.handleSubmitListen = this.handleSubmitListen.bind(this);
	this.handleSubmitUpload = this.handleSubmitUpload.bind(this);
	this.handleSubmitDump = this.handleSubmitDump.bind(this);
	this.handleSubmitClear = this.handleSubmitClear.bind(this);
  }




 handleSubmitListen(event) {
	let requestOptions = {
	  method: 'GET',
	};

	let url="./api/toggleListen" 
	let value;
	if (this.state.listening === "Listening") {
		value = "Stopped";

	 } else if  (this.state.listening === "Stopped") {
		 value = "Listening";

	 } else {
		 value = "Listening";
	 }
	fetch(url, requestOptions)
	.then(() => this.setState({
		 "listening" : value, // Listenning status = Listenning / Stoped Listenning
	 }))
	 .then(() => {
			if (this.state.listening === "Stopped") {
				 this.setState({
					"showUpload" : true,
				 })
			 } else {
				this.setState({
					"showUpload" : false,
					"showDump": false,
					"showClear": false,
				 })
		 }
	 })

	 .catch(error => console.log('error', error));
	 
	event.preventDefault();
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
		  "uploadStatus" : "Uploaded",
		  "showDump": true
	 }))
	  .catch(error => alert('error', error));
	event.preventDefault();
  }



handleSubmitDump(event) {
	let requestOptions = {
	  method: 'GET',
	};
	
	let url="/api/dump" 
	this.setState({"dumpStatus" : "Dumping"});
	fetch(url, requestOptions)
	  .then(response => response.text())
	  .then(() => 	 this.setState({
		  "dumpStatus" : "Dumped",
		  "showClear": true
	 }))
	  .catch(error => alert('error', error));
	event.preventDefault();
  }


 handleSubmitClear(event) {
	let requestOptions = {
	  method: 'GET',
	};
	
	let url="/api/clear" 
	this.setState({"clearStatus" : "Cleaning"});
	fetch(url, requestOptions)
	  .then(response => response.text())
	  .then(() => 	 this.setState({
		  "clearStatus" : "Cleaned"
	 }))
	  .catch(error => alert('error', error));
	event.preventDefault();
  }



    render() { 
		console.log(this.state.showUpload);
		return(
			<div id="wrapper">
				<div className="formWrap">
					<RoundButton 
						listening={this.state.listening} 
						submit={this.handleSubmitListen}
					/>
					<Button 
						listening={this.state.listening} 
						submit={this.handleSubmitUpload} 
						value={this.state.uploadStatus} 
						show={this.state.showUpload}	
					/>

				  
					<Button 
						listening={this.state.listening} 
						submit={this.handleSubmitDump} 
						value={this.state.dumpStatus} 
						show={this.state.showDump}	
					/>
					<Button 
						listening={this.state.listening} 
						submit={this.handleSubmitClear} 
						value={this.state.clearStatus} 
						show={this.state.showClear}
					/>

				</div>
			</div>
	);

  }
  
}

export default App;
