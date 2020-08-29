import React from 'react';

function button(props) {
	if (props.shonw){
	 return(
		 <form className="formWrap flat" onSubmit={props.submit}>
		 <input 
			 className="btn flat" 
			 id={props.listening? "listening" : null} 
			 type="submit" 
			 value={props.value}
		 />
	  </form> )
	} else {
	return 	null;
	}
}
	



export default button;


