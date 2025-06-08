Name: pg_conda_%{_pg_version}
Version: %{_version}
Release: 1%{?dist}
License: MIT
URL: https://github.com/JeanChristopheMorinPerso/pg_conda
Summary: PostgreSQL extension that adds types and functions for the conda ecosystem.

Requires:       postgresql%{_pg_version}-server

%description
pg_conda is a PostgreSQL extension that adds types and functions for the conda ecosystem to your PostgreSQL database.

%install
%{__rm} -rf %{buildroot}
install -d %{buildroot}/usr/pgsql-%{_pg_version}/lib/
install -d %{buildroot}/usr/pgsql-%{_pg_version}/share/extension/
install -m 755 %{_sourcedir}/pg_conda_%{_pg_version}/usr/pgsql-%{_pg_version}/lib/pg_conda.so %{buildroot}/usr/pgsql-%{_pg_version}/lib/
install -m 644 %{_sourcedir}/pg_conda_%{_pg_version}/usr/pgsql-%{_pg_version}/share/extension/pg_conda*.sql %{buildroot}/usr/pgsql-%{_pg_version}/share/extension/
install -m 644 %{_sourcedir}/pg_conda_%{_pg_version}/usr/pgsql-%{_pg_version}/share/extension/pg_conda.control %{buildroot}/usr/pgsql-%{_pg_version}/share/extension/

%files
/usr/pgsql-%{_pg_version}/lib/pg_conda.so
/usr/pgsql-%{_pg_version}/share/extension/pg_conda.control
/usr/pgsql-%{_pg_version}/share/extension/pg_conda*sql
