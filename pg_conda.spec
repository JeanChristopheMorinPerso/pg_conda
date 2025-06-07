%define pg_version %{?postgresql_version:%{postgresql_version}}%{!?postgresql_version:17}

Name: pg_conda
Version: 1.0.0
Release: 1%{?dist}
Summary: PostgreSQL extension that adds types and functions for the conda ecosystem.
License: MIT
URL: https://github.com/JeanChristopheMorinPerso/pg_conda

Requires:       postgresql%{pg_version}-server

%description

%install
%{__rm} -rf %{buildroot}
install -d %{buildroot}/usr/pgsql-%{pg_version}/lib/
install -d %{buildroot}/usr/pgsql-%{pg_version}/share/extension/
install -m 755 %{_sourcedir}/pg_conda_%{pg_version}/usr/pgsql-%{pg_version}/lib/pg_conda.so %{buildroot}/usr/pgsql-%{pg_version}/lib/
install -m 644 %{_sourcedir}/pg_conda_%{pg_version}/usr/pgsql-%{pg_version}/share/extension/pg_conda*.sql %{buildroot}/usr/pgsql-%{pg_version}/share/extension/
install -m 644 %{_sourcedir}/pg_conda_%{pg_version}/usr/pgsql-%{pg_version}/share/extension/pg_conda.control %{buildroot}/usr/pgsql-%{pg_version}/share/extension/

%files
/usr/pgsql-%{pg_version}/lib/pg_conda.so
/usr/pgsql-%{pg_version}/share/extension/pg_conda.control
/usr/pgsql-%{pg_version}/share/extension/pg_conda*sql
