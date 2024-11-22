import Link from 'next/link';
import Image from 'next/image';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faUser, faHome, faChartLine } from '@fortawesome/free-solid-svg-icons'

export default function Header () {
  return (
    (<header className="bg-primary text-white py-4 border-secondary border-opacity-50 shadow-md border-b-2">
      <div className="container mx-auto flex items-center justify-between px-2">
				{/*<h1 className="text-2xl font-bold">jammy</h1>*/}

				<div className="flex gap-2 items-center">
				<Image src="/jammy.png" alt="jammy logo" width={60} height={60} />
				<h1 className="text-2xl font-bold">jammy</h1>
				</div>

        <nav className="flex space-x-4">
					<Link
						href="/"
						key="home"
						className="text-lg font-semibold bg-secondary px-4 py-2 rounded-full hover:bg-primary-med transition-all ease-out duration-300 transform hover:scale-110">
						<FontAwesomeIcon icon={faHome} className="fa-fw" />
					</Link>

					<Link
						href="/Jamstats"
						key="jamstats"
						className="text-lg font-semibold bg-secondary px-4 py-2 rounded-full hover:bg-primary-med transition-all ease-out duration-300 transform hover:scale-110">
						<FontAwesomeIcon icon={faChartLine} className="fa-fw" />
					</Link>

            <Link
              href="/account"
              key="account"
              className="text-lg font-semibold bg-secondary px-4 py-2 rounded-full hover:bg-primary-med transition-all ease-out duration-300 transform hover:scale-110">

							<FontAwesomeIcon icon={faUser} className="fa-fw" />

            </Link>
        </nav>
      </div>
    </header>)
  );
};
