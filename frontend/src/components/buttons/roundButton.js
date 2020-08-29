import React from 'react';


const roundButton =  (props) => (
	<form className="formWrap" onSubmit={props.submit}>
		 <input 
			className="btn round" 
			id={props.listening} 
			type="submit" 
			value={props.listening}
		  />
	  </form> 
)

export default roundButton;
