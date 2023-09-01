const storeToken = (token: string) => {
	document.cookie = `token=${token}; path=/; samesite=strict`;
};

export { storeToken };
