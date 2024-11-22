// We should have a struct that has a `token` and a bool `isValid` to check if the token is valid or not

class Jammy {
	token: string;
	isValid: boolean;

	constructor() {
		this.token = "";
		this.isValid = false;
	}
}

export default new Jammy();
