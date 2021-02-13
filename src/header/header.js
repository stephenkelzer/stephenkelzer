import './header.css';

const Header = () => {

    return (
        <header>
            <div id="SocialBar">
                <div className="icons">
                    <a className="icon" href="https://www.facebook.com/StephenKelzer" title="Facebook" target="_blank" rel="noreferrer">
                        <i className="bi bi-facebook"></i>
                    </a>
                    <a className="icon" href="https://twitter.com/TheSteveKelzer" title="Twitter" target="_blank" rel="noreferrer">
                        <i className="bi bi-twitter"></i>
                    </a>
                    <a className="icon" href="https://www.linkedin.com/in/stephenkelzer" title="LinkedIn" target="_blank" rel="noreferrer">
                        <i className="bi bi-linkedin"></i>
                    </a>
                    <a className="icon" href="http://stackoverflow.com/users/1689788/stephenkelzer" title="Stack Overflow" target="_blank" rel="noreferrer">
                        <i className="bi bi-code-slash"></i>
                    </a>
                </div>
            </div>
            <div id="MadeBy" title="This site has been designed and developed by Stephen Kelzer">
                <span>
                    <span className="font1">made</span><span className="font2">by</span><span className="font1">stephen</span>
                </span>
            </div>
        </header>
    );
};

export default Header;
export { Header };