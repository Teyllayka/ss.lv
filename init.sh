#!/bin/bash

# Stop all services first
docker compose down

# Create directories for certbot
mkdir -p ./certbot/conf
mkdir -p ./certbot/www

# Create a non-SSL nginx.conf to verify basic connectivity first
cat > ./nginx.conf << EOL
events {}

http {
    server_tokens off;

    proxy_buffer_size          128k;
    proxy_buffers              4 256k;
    proxy_busy_buffers_size    256k;
    large_client_header_buffers 4 16k;

    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # Define your upstreams, ensure their names match your container names
    upstream backend_servers  { server backend:80; }
    upstream frontend_servers { server frontend:3000; }
    upstream chat_servers     { server chat:4000; }

    # HTTP server - we'll handle only HTTP first
    server {
        listen 80;
        server_name ad-ee.tech www.ad-ee.tech;

        # Certbot challenge location
        location /.well-known/acme-challenge/ {
            root /var/www/certbot;
        }

        location /chat/ {
            rewrite ^/chat/(.*)$ /$1 break;

            proxy_pass         http://chat_servers;
            proxy_http_version 1.1;
            proxy_set_header   Upgrade           \$http_upgrade;
            proxy_set_header   Connection        "upgrade";
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location = /backend {
            # internally rewrite URI from "/backend" → "/"
            rewrite ^ / break;

            proxy_pass         http://backend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }

        location /backend/ {
            rewrite ^/backend/(.*)$ /$1 break;
            proxy_pass         http://backend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }

        location ^~ /_app/immutable/ {
            proxy_pass         http://frontend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location /api/ {
            proxy_pass         http://frontend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location / {
            proxy_pass         http://frontend_servers/;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }
    }
}
EOL

# Start all services
echo "Starting services with HTTP-only configuration to verify connectivity..."
docker compose up -d

echo "Waiting for services to stabilize..."
sleep 10

# Check if nginx is running correctly
if docker ps | grep nginx_proxy | grep -q "Up"; then
    echo "Nginx is running correctly."
    echo "Now let's proceed to obtain SSL certificates..."
else
    echo "ERROR: Nginx failed to start. Check logs with 'docker logs nginx_proxy'"
    exit 1
fi

# Use docker exec to run certbot command in the existing certbot container
echo "Attempting to obtain SSL certificate for ad-ee.tech..."
docker compose exec certbot /bin/sh -c \
  "certbot certonly \
     --webroot --webroot-path=/var/www/certbot \
     --email your-email@example.com --agree-tos --no-eff-email \
     --force-renewal \
     -d ad-ee.tech -d www.ad-ee.tech"

# Check if certificates were obtained successfully
if [ -d "./certbot/conf/live/ad-ee.tech" ]; then
    echo "Certificates obtained successfully!"

    # Update nginx.conf to include SSL
    cat > ./nginx.conf << EOL
events {}

http {
    server_tokens off;

    proxy_buffer_size          128k;
    proxy_buffers              4 256k;
    proxy_busy_buffers_size    256k;
    large_client_header_buffers 4 16k;

    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # Define your upstreams with better names
    upstream backend_servers  { server backend:80; }
    upstream frontend_servers { server frontend:3000; }
    upstream chat_servers     { server chat:4000; }

    # HTTP server - redirects to HTTPS
    server {
        listen 80;
        server_name ad-ee.tech www.ad-ee.tech;

        # Certbot challenge location
        location /.well-known/acme-challenge/ {
            root /var/www/certbot;
        }

        # Redirect all HTTP traffic to HTTPS
        location / {
            return 301 https://\$host\$request_uri;
        }
    }

    # HTTPS server
    server {
        listen 443 ssl;
        server_name ad-ee.tech www.ad-ee.tech;

        # SSL certificates managed by Certbot
        ssl_certificate /etc/letsencrypt/live/ad-ee.tech/fullchain.pem;
        ssl_certificate_key /etc/letsencrypt/live/ad-ee.tech/privkey.pem;

        # SSL configuration
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_prefer_server_ciphers on;
        ssl_ciphers "EECDH+AESGCM:EDH+AESGCM:AES256+EECDH:AES256+EDH";
        ssl_session_cache shared:SSL:10m;
        ssl_session_timeout 1d;
        ssl_session_tickets off;

        # HSTS (optional, but recommended)
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

        location /chat/ {
            rewrite ^/chat/(.*)$ /$1 break;

            proxy_pass         http://chat_servers;
            proxy_http_version 1.1;
            proxy_set_header   Upgrade           \$http_upgrade;
            proxy_set_header   Connection        "upgrade";
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location = /backend {
            # internally rewrite URI from "/backend" → "/"
            rewrite ^ / break;

            proxy_pass         http://backend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }

        location /backend/ {
            rewrite ^/backend/(.*)$ /$1 break;
            proxy_pass         http://backend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }

        location ^~ /_app/immutable/ {
            proxy_pass         http://frontend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location /api/ {
            proxy_pass         http://frontend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location / {
            proxy_pass         http://frontend_servers/;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }
    }
}
EOL

    # Restart nginx to apply SSL configuration
    echo "Restarting Nginx with SSL configuration..."
    docker compose restart nginx
    echo "Done! Your system should now be running with HTTPS enabled."
    echo "Visit https://ad-ee.tech to verify."
else
    echo "Failed to obtain certificates. Check for errors above."
    exit 1
fi

echo "Process complete! Your site should now be accessible via HTTPS."
root@ubuntu-s-2vcpu-4gb-fra1-01:~/ss.lv# cat init-certbot.sh
#!/bin/bash

# Stop all services first
docker compose down

# Create directories for certbot
mkdir -p ./certbot/conf
mkdir -p ./certbot/www

# Create a non-SSL nginx.conf to verify basic connectivity first
cat > ./nginx.conf << EOL
events {}

http {
    server_tokens off;

    proxy_buffer_size          128k;
    proxy_buffers              4 256k;
    proxy_busy_buffers_size    256k;
    large_client_header_buffers 4 16k;

    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # Define your upstreams, ensure their names match your container names
    upstream backend_servers  { server backend:80; }
    upstream frontend_servers { server frontend:3000; }
    upstream chat_servers     { server chat:4000; }

    # HTTP server - we'll handle only HTTP first
    server {
        listen 80;
        server_name ad-ee.tech www.ad-ee.tech;

        # Certbot challenge location
        location /.well-known/acme-challenge/ {
            root /var/www/certbot;
        }

        location /chat/ {
            rewrite ^/chat/(.*)$ /$1 break;

            proxy_pass         http://chat_servers;
            proxy_http_version 1.1;
            proxy_set_header   Upgrade           \$http_upgrade;
            proxy_set_header   Connection        "upgrade";
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location = /backend {
            # internally rewrite URI from "/backend" → "/"
            rewrite ^ / break;

            proxy_pass         http://backend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }

        location /backend/ {
            rewrite ^/backend/(.*)$ /$1 break;
            proxy_pass         http://backend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }

        location ^~ /_app/immutable/ {
            proxy_pass         http://frontend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location /api/ {
            proxy_pass         http://frontend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location / {
            proxy_pass         http://frontend_servers/;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }
    }
}
EOL

# Start all services
echo "Starting services with HTTP-only configuration to verify connectivity..."
docker compose up -d

echo "Waiting for services to stabilize..."
sleep 10

# Check if nginx is running correctly
if docker ps | grep nginx_proxy | grep -q "Up"; then
    echo "Nginx is running correctly."
    echo "Now let's proceed to obtain SSL certificates..."
else
    echo "ERROR: Nginx failed to start. Check logs with 'docker logs nginx_proxy'"
    exit 1
fi

# Use docker exec to run certbot command in the existing certbot container
echo "Attempting to obtain SSL certificate for ad-ee.tech..."
docker compose exec certbot certbot certonly --webroot \
  --webroot-path=/var/www/certbot \
  --email your-email@example.com --agree-tos --no-eff-email \
  --force-renewal \
  -d ad-ee.tech -d www.ad-ee.tech

# Check if certificates were obtained successfully
if [ -d "./certbot/conf/live/ad-ee.tech" ]; then
    echo "Certificates obtained successfully!"

    # Update nginx.conf to include SSL
    cat > ./nginx.conf << EOL
events {}

http {
    server_tokens off;

    proxy_buffer_size          128k;
    proxy_buffers              4 256k;
    proxy_busy_buffers_size    256k;
    large_client_header_buffers 4 16k;

    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    # Define your upstreams with better names
    upstream backend_servers  { server backend:80; }
    upstream frontend_servers { server frontend:3000; }
    upstream chat_servers     { server chat:4000; }

    # HTTP server - redirects to HTTPS
    server {
        listen 80;
        server_name ad-ee.tech www.ad-ee.tech;

        # Certbot challenge location
        location /.well-known/acme-challenge/ {
            root /var/www/certbot;
        }

        # Redirect all HTTP traffic to HTTPS
        location / {
            return 301 https://\$host\$request_uri;
        }
    }

    # HTTPS server
    server {
        listen 443 ssl;
        server_name ad-ee.tech www.ad-ee.tech;

        # SSL certificates managed by Certbot
        ssl_certificate /etc/letsencrypt/live/ad-ee.tech/fullchain.pem;
        ssl_certificate_key /etc/letsencrypt/live/ad-ee.tech/privkey.pem;

        # SSL configuration
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_prefer_server_ciphers on;
        ssl_ciphers "EECDH+AESGCM:EDH+AESGCM:AES256+EECDH:AES256+EDH";
        ssl_session_cache shared:SSL:10m;
        ssl_session_timeout 1d;
        ssl_session_tickets off;

        # HSTS (optional, but recommended)
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

        location /chat/ {
            rewrite ^/chat/(.*)$ /$1 break;

            proxy_pass         http://chat_servers;
            proxy_http_version 1.1;
            proxy_set_header   Upgrade           \$http_upgrade;
            proxy_set_header   Connection        "upgrade";
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location = /backend {
            # internally rewrite URI from "/backend" → "/"
            rewrite ^ / break;

            proxy_pass         http://backend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }

        location /backend/ {
            rewrite ^/backend/(.*)$ /$1 break;
            proxy_pass         http://backend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }

        location ^~ /_app/immutable/ {
            proxy_pass         http://frontend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location /api/ {
            proxy_pass         http://frontend_servers;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
            proxy_read_timeout 3600s;
            proxy_buffering    off;
        }

        location / {
            proxy_pass         http://frontend_servers/;
            proxy_set_header   Host              \$host;
            proxy_set_header   X-Real-IP         \$remote_addr;
            proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
            proxy_set_header   X-Forwarded-Proto \$scheme;
        }
    }
}
EOL

    # Restart nginx to apply SSL configuration
    echo "Restarting Nginx with SSL configuration..."
    docker compose restart nginx
    echo "Done! Your system should now be running with HTTPS enabled."
    echo "Visit https://ad-ee.tech to verify."
else
    echo "Failed to obtain certificates. Check for errors above."
    exit 1
fi

echo "Process complete! Your site should now be accessible via HTTPS."